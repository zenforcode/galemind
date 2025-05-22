use axum::{Router, routing::get};
use clap::Command;

#[tokio::main]
async fn main() {
    let matches = Command::new("galemind")
        .version("0.1")
        .author("Zenforcode Team <team@zenforcode.com>")
        .about("GaleMind ML Inference Server v0.1")
        .subcommand(Command::new("start").about("Start the server"))
        .get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("start", _sub_matches)) => {
            init_http_server().await;
        }
        _ => {
            println!("Use --help for usage.");
        }
    }
}

async fn init_http_server() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
