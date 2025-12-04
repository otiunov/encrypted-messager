mod model;
mod error;
mod config;

use std::env::args;
use std::io;

use anyhow::Result;

use crate::model::{UserConfig};
use crate::config::load_or_init_config;


enum Mode {
    Server,
    Client,
}

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    let mut app_args = args().skip(1);

    let mode = match app_args.next().as_deref() {
        Some("server") => Mode::Server,
        Some("client") => Mode::Client,
        Some(other) => {
            eprintln!("Unknown mode: {other}");
            eprintln!("Usage: encrypted-messenger <server|client>");
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: encrypted-messenger <server|client>");
            std::process::exit(1);
        }
    };

    let cfg = load_or_init_config()?;

    match mode {
        Mode::Server => {
            println!("Running in server mode ...");
            run_server(&cfg)?
        }
       Mode::Client => {
            println!("Running in client mode ...");
            run_client(&cfg)?
        },
    };

    Ok(())
}

fn run_client(cfg: &UserConfig) -> Result<()> {
    println!("Welcome back: {}", cfg.username);
    
    println!("Enter peer username:");
    let mut peer = String::new();
    io::stdin().read_line(&mut peer)?;
    let peer = peer.trim().to_string();

    // loop {
    //     print!("> ");
    //     io::stdout().flush()?;

    //     let mut line = String::new();
    //     io::stdin().read_line(&mut line)?;
    //     let body = line.trim().to_string();

    //     if body.is_empty() { continue; }

    //     let msg = PlainMessage {
    //         from: username.clone(),
    //         to: peer.clone(),
    //         body,
    //         direction: MessageDirection::Outgoing,
    //     };

    //     println!("Constructed message: {:?}", msg);
    // }
    Ok(())
}

fn run_server(cfg: &UserConfig) -> Result<()> {
    Ok(())
}
