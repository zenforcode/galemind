use clap::Command;

fn main() {
    let matches = Command::new("galemind")
        .version("0.1")
        .author("Zenforcode Team <team@zenforcode.com>")
        .about("GaleMind ML Inference Server v0.1")
        .subcommand(Command::new("start").about("Start the server"))
        .get_matches();

    // Handle the subcommands
    match matches.subcommand() {
        Some(("start", _sub_matches)) => {
            println!("Server started with success!");
        }
        _ => {
            println!("Use --help for usage.");
        }
    }
}
