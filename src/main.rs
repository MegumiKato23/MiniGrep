use std::sync::Arc;
use std::process;
use clap::Parser;
use minigrep::Config;


#[tokio::main]
async fn main() {
    let args = Config::parse();
    let pattern = Arc::new(args.pattern);
    let search_path = args.path;

    if let Err(e) = minigrep::run(search_path, pattern.to_string()).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}