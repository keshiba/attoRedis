mod request;
mod response;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use clap::Parser;
use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;

use request::Request;
use crate::response::Response;

struct Database {
    map: Mutex<HashMap<String, String>>
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServerConfig {
    #[arg(short, long, default_value = "127.0.0.1")]
    server_host: String,

    #[arg(short, long, default_value_t = 10129)]
    port: u16,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = ServerConfig::parse();
    let host_url = format!("{}:{}", args.server_host, args.port);

    let db = Arc::new(Database {
        map: Mutex::new(HashMap::new())
    });

    match TcpListener::bind(&host_url).await {
        Ok(listener) => {
            info!("Server running at {}", &host_url);
            loop {
                let (stream, socket_addr) = listener.accept().await.unwrap();
                let client_ip = socket_addr.ip();
                let client_port = socket_addr.port();
                info!("Accepted connection from incoming client {}:{}", client_ip, client_port);

                let db = db.clone();

                tokio::spawn(async move {
                    let mut lines = Framed::new(stream, LinesCodec::new());

                    while let Some(request_line) = lines.next().await {
                        match request_line {
                            Ok(line) => {
                                let response = handle_request(&line, &db);
                                if let Err(e) = lines.send(response.serialize().as_str()).await {
                                    error!("error sending response: {}", e);
                                }
                            }
                            Err(e) => {
                                error!("error decoding request from client. error: {:?}", e);
                            }
                        }
                    }
                });
            }
        }
        Err(e) => {
            error!("Unable to start server at {}. Error: {}", host_url, e)
        }
    }
}

fn handle_request(request_str: &str, db: &Arc<Database>) -> Response {
    let request = match Request::parse(request_str) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: format!("InvalidRequest: {}", e) }
    };

    let mut db = db.map.lock().unwrap();
    match request {
        Request::Echo { msg } => {
            Response::Echo { msg }
        },
        Request::Get { key } => match db.get(&key) {
            Some(value) => Response::Value {
                value: value.clone(),
            },
            None => Response::Error {
                msg: "key not found".into()
            }
        }
        Request::Set { key, value } => {
            db.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
            }
        }
        Request::Keys => {
            let keys: Vec<String> = db.keys().map(|key| key.clone()).collect();
            Response::Keys {
                keys
            }
        }
    }
}

