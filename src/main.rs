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
use std::sync::{Arc, RwLock};

use dotenv::dotenv;
use iron::{Iron, Chain};
use logger::Logger;
use mount::Mount;
use simplelog::{Config, LogLevelFilter, TermLogger, CombinedLogger};
use staticfile::Static;
use ws::{listen, Message};

struct Model {
    value: u64,
}

impl Model {
    pub fn new(starting_value: u64) -> Model {
        Model { value: starting_value }
    }

    pub fn increment(&mut self) {
        self.value = self.value + 1
    }

    pub fn decrement(&mut self) {
        if self.value > 0 {
            self.value = self.value - 1
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

fn main() {
    dotenv().ok();
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    info!("Logger configured");

    let model = Model::new(0);
    let model_ref: Arc<RwLock<Model>> = Arc::new(RwLock::new(model));

    let iron_thread = thread::spawn(||{
        let server_address = env::var("address").expect("\"address\" in environment variables");
        info!("starting server at {}", server_address);

        Iron::new(chain()).http(server_address).unwrap();
    });

    let ws_model_ref = model_ref.clone();
    let ws_thread = thread::spawn(move ||{
        let socket_address = env::var("socket").expect("\"socket\" in environment variables");
        info!("starting web socket at {}", socket_address);

        if let Err(error) = listen(socket_address, |out| {
            let handler_model_ref = ws_model_ref.clone();

            move |msg: Message| {
                info!("Server got message '{}'. ", msg);
                let message: String = msg.to_string();
                let mut model = handler_model_ref.write().unwrap();
                match message.as_ref() {
                    "increment" => model.increment(),
                    "decrement" => model.decrement(),
                    _ => {
                        /* do nothing */
                    },
                }
                out.broadcast(model.value().to_string())
            }

        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to {:?}", error);
}
    });

    iron_thread.join().unwrap();
    ws_thread.join().unwrap();
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
