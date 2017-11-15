extern crate dotenv;
extern crate iron;
#[macro_use] extern crate log;
extern crate logger;
extern crate mount;
extern crate simplelog;
extern crate staticfile;
extern crate ws;

use std::env;
use std::path::Path;
use std::thread;

use dotenv::dotenv;
use iron::{Iron, Chain};
use logger::Logger;
use mount::Mount;
use simplelog::{Config, LogLevelFilter, TermLogger, CombinedLogger};
use staticfile::Static;
use ws::listen;

fn main() {
    dotenv().ok();
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    info!("Logger configured");

    let iron_thread = thread::spawn(||{
        let server_address = env::var("address").expect("\"address\" in environment variables");
        info!("starting server at {}", server_address);

        Iron::new(chain()).http(server_address).unwrap();
    });

    let ws_thread = thread::spawn(||{
        let socket_address = env::var("socket").expect("\"socket\" in environment variables");
        info!("starting web socket at {}", socket_address);

        if let Err(error) = listen(socket_address, |out| {

            // The handler needs to take ownership of out, so we use move
            move |msg| {

                // Handle messages received on this connection
                info!("Server got message '{}'. ", msg);

                // Use the out channel to send messages back
                out.send(msg)
            }

        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
}
    });

    iron_thread.join();
    ws_thread.join();
}

fn chain() -> Chain {
    let mut chain = Chain::new(mount());
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    chain
}

fn mount() -> Mount {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("static/")));

    mount
}
