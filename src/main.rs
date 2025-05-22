mod rest_server;
use crate::rest_server::RestServerBuilder;
use clap::Command;
#[tokio::main]
async fn main() {
    let matches = Command::new("galemind")
        .version("0.1")
        .author("Zenforcode Team <team@zenforcode.com>")
        .about("GaleMind ML Inference Server v0.1")
        .subcommand(Command::new("start").about("Start the server"))
        .get_matches();

    match matches.subcommand() {
        Some(("start", _sub_matches)) => {
            println!("Server started with success!");
            if let Err(e) = RestServerBuilder::new("127.0.0.1:3000").build().await {
                eprintln!("Server failed to start: {}", e);
            }
        }
        _ => {
            println!("Use --help for usage.");
        }
    }
}
