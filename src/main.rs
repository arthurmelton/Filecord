mod server;
use crate::server::new::new;
mod utils;

use std::env;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or("80".to_string())
    ))
        .unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            new(stream.unwrap());
        });
    }
}
