mod model;

use std::error::Error;
use std::env::args;
use std::io::{self, Write};

use crate::model::{PlainMessage, UserConfig, MessageDirection};


enum Mode {
    Server,
    Client,
}

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut app_args = args().skip(1);
    
    let mode = match app_args.next().as_deref() {
        Some("server") => "server",
        Some("client") => "client",
        _ => {
            eprintln!("Usage: encrypted-messenger <server|client>");
            std::process::exit(1);
        }
    };

    match mode {
        "server" => {
            println!("Running in server mode ...");
        }
        "client" => {
            println!("Running in client mode ...");
            run_client()?
        }
        _ => unreachable!(),
    };

    Ok(())
}

fn run_client() -> Result<(), Box<dyn Error>> {
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    println!("Enter peer username:");
    let mut peer = String::new();
    io::stdin().read_line(&mut peer)?;
    let peer = peer.trim().to_string();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let body = line.trim().to_string();

        if body.is_empty() { continue; }

        let msg = PlainMessage {
            from: username.clone(),
            to: peer.clone(),
            body,
            direction: MessageDirection::Outgoing,
        };

        println!("Constructed message: {:?}", msg);
    }
}
