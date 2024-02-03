mod request;
mod response;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use futures::SinkExt;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;

use request::Request;
use crate::response::Response;

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

    match TcpListener::bind(&host_url).await {
        Ok(listener) => {
            info!("Server running at {}", &host_url);
            loop {
                let (stream, socket_addr) = listener.accept().await.unwrap();
                let client_ip = socket_addr.ip();
                let client_port = socket_addr.port();
                info!("Accepted connection from incoming client {}:{}", client_ip, client_port);

                tokio::spawn(async move {
                    let mut lines = Framed::new(stream, LinesCodec::new());

                    while let Some(request_line) = lines.next().await {
                        match request_line {
                            Ok(line) => {
                                let response = handle_request(&line);
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

fn handle_request(request_str: &str) -> Response {
    let request = match Request::parse(request_str) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: format!("InvalidRequest: {}", e) }
    };

    match request {
        Request::Hello => {
            Response::Hello
        },
        Request::Get { key } => {
            Response::Value {
                key: key.clone(),
                value: format!("Here's your data at {}", &key),
            }
        }
        Request::Set { key, value } => {
            Response::Set {
                key: key.clone(),
                value: format!("I'll keep your data safe {}:{}", &key, &value),
                previous: None,
            }
        }
    }
}

