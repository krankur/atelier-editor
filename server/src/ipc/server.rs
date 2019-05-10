//! Contains the IPC server for visualizing Entities in a running game

use std;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use serde_json;

pub struct IPCServer {
    host: String,
    port: String
}

impl IPCServer {
    /// Creates and returns a new IPCServer
    pub fn new(host: String, port: String) -> IPCServer {
        IPCServer {
            host,
            port
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        println!("Starting IPC server...");
        let listener = TcpListener::bind(self.host.clone() + ":" + &self.port).expect("Unable to bind IPC server");
        loop {
            for stream in listener.incoming() {
                IPCServer::handle_client(stream?);
            }
        }
    }

    fn handle_client(mut client: TcpStream) {
        let mut buf: String = String::new();
        loop {
            let _bytes_read = client.read_to_string(&mut buf);
            let message: serde_json::Value = serde_json::from_str(&buf).unwrap();
            println!("Message was: {:#?}", message);
        }
    }
}