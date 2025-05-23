use clap::Command;
use rest_server::RestServerBuilder;
use tokio::select;

#[tokio::main]
async fn main() {
    let matches = Command::new("galemind")
        .version("0.1")
        .author("Zenforcode Team <team@zenforcode.com>")
        .about("GaleMind ML Inference Server v0.1")
        .subcommand(Command::new("start").about("Start the server"))
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            println!("Launching REST and gRPC servers...");

            let rest_task = tokio::spawn(async {
                RestServerBuilder::configure("127.0.0.1:3000").start().await
            });
            // Wait for either to fail or finish
            select! {
                res = rest_task => {
                    if let Err(e) = res {
                        eprintln!("GaleMind server failed: {:?}", e);
                    }
                }
            }
        }
        _ => {
            println!("Use --help for usage.");
        }
    }
}
