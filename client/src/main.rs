extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ClientConfig {
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

    let args = ClientConfig::parse();
    let host_url = format!("{}:{}", args.server_host, args.port);
    println!("Connecting to : {}", &host_url);

    let mut tcp_stream = TcpStream::connect(&host_url).await.unwrap();
    let (reader, writer) = tcp_stream.split();

    let mut in_stream = FramedRead::new(reader, LinesCodec::new());
    let mut out_stream = FramedWrite::new(writer, LinesCodec::new());

    let mut stdout = FramedWrite::new(tokio::io::stdout(), LinesCodec::new());
    let mut stdin = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
    // let mut stdin = stdin.map(|i| i.map(BytesMut::freeze));

    loop {
        tokio::select! {
           input = stdin.next() => {
                if let Some(Ok(input)) = input {
                    out_stream.send(input).await.unwrap();
                } else {
                    break;
                }
            },
            response = in_stream.next() => {
                if let Some(Ok(msg)) = response {
                    stdout.send(msg).await.unwrap();
                } else {
                    break;
                }

            }
        }
    }
}
