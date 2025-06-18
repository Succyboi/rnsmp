pub mod consts;
pub mod server;

use std::{env::args, thread::sleep};

use crate::server::Server;

fn main() {
    let pattern = args().nth(1).expect(&format!("{log_invalid} {log_help}", log_invalid = consts::LOG_INVALID_ARGS, log_help = consts::LOG_HELP));

    match pattern.as_str() {
        "help" => help(),
        "about" => about(),
        "credits" => credits(),
        "license" => license(),
        "run" => run(),
        _ => panic!("{log_invalid} {log_help}", log_invalid = consts::LOG_INVALID_ARGS, log_help = consts::LOG_HELP)
    }
}

fn help() {
    println!("\"about\" - About {}.", consts::NAME);
    println!("\"credits\" - Who made this application.");
    println!("\"license\" - The license for this application.\n");

    println!("\"run <port>\" - Run the server on the specified port, defaults to port {}.", consts::DEFAULT_PORT);
}

fn about() {
    println!("{name} v{version} ({build_id}) {build_type} - {description}", name = consts::NAME, version = consts::VERSION, build_id = consts::BUILD_ID, build_type = consts::BUILD_TYPE, description = consts::DESCRIPTION);
    println!("By {}\n", consts::AUTHORS);

    println!("Licensed under {}.\n", consts::LICENSE_NAME);

    println!("{}", consts::MOTD);
}

fn credits() {
    println!("{name} - {description}", name = consts::NAME, description = consts::DESCRIPTION);
    println!("By {}\n", consts::AUTHORS);

    println!("{}", consts::CREDITS);
}

fn license() {
    println!("Licensed under {}.", consts::LICENSE_NAME);
    println!("{}", consts::LICENSE_CONTENTS);
}

fn run() {
    about();

    let port = match args().nth(2) {
        Some(p) => p,
        None => {
            println!("No port specified, using default port {}.", consts::DEFAULT_PORT);
            format!("{}", consts::DEFAULT_PORT)
        }
    };

    println!("Initializing server.");
    let mut server = match Server::new(&port) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };
    println!("Initialized server succesfully.");

    println!("Running.");
    loop {
        match server.tick() {
            Ok(()) => (),
            Err(e) => println!("Tick {tick} failed: {error}", tick = server.tick, error = e)
        }

        sleep(*consts::DEFAULT_TICK_DURATION);
    }
}