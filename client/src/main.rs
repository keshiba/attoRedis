extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use clap::Parser;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

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

    let mut stdout = FramedWrite::new(tokio::io::stdout(), BytesCodec::new());
    let stdin = FramedRead::new(tokio::io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(BytesMut::freeze));
    let mut tcp_stream = TcpStream::connect(&host_url).await.unwrap();

    let (reader, writer) = tcp_stream.split();
    let mut stream = FramedRead::new(reader, BytesCodec::new());
    let mut sink = FramedWrite::new(writer, BytesCodec::new());

    loop {
        tokio::select! {
           input = stdin.next() => {
                if let Some(Ok(input)) = input {

                    println!("Sending: {}", std::str::from_utf8(&input).unwrap().to_string());
                    sink.send(input).await.unwrap();
                } else {
                    break;
                }
            },
            response = stream.next() => {
                if let Some(Ok(msg)) = response {
                    stdout.send(msg).await.unwrap();
                } else {
                    break;
                }

            }
        }
    }
}
