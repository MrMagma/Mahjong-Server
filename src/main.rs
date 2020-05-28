#[macro_use] extern crate log;
extern crate env_logger;

use std::collections::HashMap;

mod game;
mod server;

fn main() {
    env_logger::init();

    let mut server = server::Server::new();

    if let Err(e) = server.run() {
        error!("Error starting server: {}", e);
    }
}
