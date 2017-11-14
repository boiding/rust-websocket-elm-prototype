#[macro_use] extern crate log;
extern crate simplelog;
extern crate logger;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use std::path::Path;

use iron::{Iron, Chain};
use staticfile::Static;
use mount::Mount;
use simplelog::{Config, LogLevelFilter, TermLogger, CombinedLogger};
use logger::Logger;

fn main() {
    dotenv().ok();
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    info!("Logger configured");

    let address = env::var("address").expect("\"address\" in environment variables");
    info!("starting server at {}", address);

    Iron::new(chain()).http(address).unwrap();
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
