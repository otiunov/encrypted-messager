mod model;
mod error;
mod config;
mod crypto;

use std::env::args;

use anyhow::Result;

use crate::model::{UserConfig};
use crate::config::load_or_init_config;
use crate::crypto::{load_or_generate_keypair, format_pub_key};


enum AppCommand {
    Server,
    Client(ClientCommand),
}

enum ClientCommand {
    Run,
    ShowKeyOnly,
}

fn main() -> Result<()> {
    run()
}

fn parse_args() -> AppCommand {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("server") => AppCommand::Server,
        Some("client") => {
            match args.next().as_deref() {
                Some("--show-key-only") => AppCommand::Client(ClientCommand::ShowKeyOnly),
                _ => AppCommand::Client(ClientCommand::Run),
            }
        }
        _ => {
            eprintln!("Usage...");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    let cfg = load_or_init_config()?;

    match parse_args() {
        AppCommand::Server => run_server(&cfg),
        AppCommand::Client(ClientCommand::ShowKeyOnly) => show_public_key(&cfg),
        AppCommand::Client(ClientCommand::Run) => run_client(&cfg),
    }
}

fn show_public_key(cfg: &UserConfig) -> Result<()> {
    let pair = load_or_generate_keypair()?;
    println!("Welcome {}, your public key:", cfg.username);
    println!("{}", format_pub_key(&pair));
    Ok(())
}

fn run_client(cfg: &UserConfig) -> Result<()> {
    // println!("Enter peer username:");
    // let mut peer = String::new();
    // io::stdin().read_line(&mut peer)?;
    // let peer = peer.trim().to_string();

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
