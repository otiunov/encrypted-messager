mod model;

use std::error::Error;
use std::env::args;

use crate::model::{PlainMessage, UserConfig, MessageDirection};


// enum Mode {
//     Server,
//     Client,
// }

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
        }
        _ => unreachable!(),
    };

    Ok(())
}
