use std::io;
use std::io::Read;
use std::net::UdpSocket;
use std::time::Duration;

extern crate hyper;

use hyper::{body::HttpBody as _, Client, Uri};

use futures::executor::block_on;

use time::Instant;
const MILLISECONDS_IN_SECOND: u64 = 1000;
const SERVER_TICK_HZ: u64 = 60;

// Macro-enabled internal crates
#[macro_use]
pub mod macros;
// Non-macro internal crates
pub mod cb_math;
pub mod cb_simulation;

fn external_ip() -> String {
    let client = Client::new();

    // Make a GET /ip to 'http://httpbin.org'
    let res = block_on(client.get(Uri::from_static("http://ipinfo.io/ip/")));

    let res = res.unwrap();

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    let mut external_ip = String::new();

    // Concatenate the body stream into a single buffer...
    let buf = block_on(hyper::body::to_bytes(res));
    let buf = buf.unwrap();

    println!("body: {:?}", buf);
    /*
    let mut response = client
        .get("http://ipinfo.io/ip/")
        .header(Connection::close())
        .send()
        .unwrap();
    let mut external_ip = String::new();
    response.read_to_string(&mut external_ip).unwrap();
    */
    return external_ip;
}

fn log_debug(s: String) {
    let debug = true;
    if debug {
        println!("DEBUG: {}", s);
    }
}

pub struct Game {
    frame_duration: Duration,
    last_frame_execution: Instant,
    socket: UdpSocket,
}

impl Game {
    pub fn new() -> Self {
        let start = Instant::now();

        let ip = external_ip();

        let frame_duration = Duration::from_millis(MILLISECONDS_IN_SECOND / SERVER_TICK_HZ);

        // Initialize server socket
        let socket;
        {
            let socket_addr = "127.0.0.1:3400";
            socket = UdpSocket::bind(socket_addr).expect("couldn't bind to address");
            socket.set_read_timeout(Some(frame_duration)).unwrap();

            println!("Server listening for clients on: {}", socket_addr);
        }

        Self {
            frame_duration: frame_duration,
            last_frame_execution: start,
            socket: socket,
        }
    }

    pub fn ready_to_run(&self) -> bool {
        let now = self.last_frame_execution - Instant::now();
        let run_game_sim = self.frame_duration <= now;

        return run_game_sim;
    }

    pub fn execute(&mut self) {
        if self.ready_to_run() {
            let mut buf = [0; 10];

            //TODO: have clients send their ip address + port, so this can send data to them
            //let clients = vec![];

            match self.socket.recv(&mut buf) {
                Ok(recieved) => {
                    log_debug(format!(
                        "recieved {} bytes: {:?}",
                        recieved,
                        &buf[..recieved]
                    ));
                }
                Err(e) => {
                    //log_debug(e.to_string())
                }
            }

            self.last_frame_execution = Instant::now();
        }
    }
}

pub fn main() {
    let mut game = Game::new();

    loop {
        game.execute();
    }
}
