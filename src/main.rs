mod client;
mod common;
mod network;
mod platform;
mod server;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  keyboard-sync server --port <PORT>");
        eprintln!("  keyboard-sync client --server-ip <IP> --port <PORT>");
        return Ok(());
    }

    match args[1].as_str() {
        "server" => server::run(args).await?,
        "client" => client::run(args).await?,
        _ => eprintln!("Unknown command. Use 'server' or 'client'"),
    }

    Ok(())
}