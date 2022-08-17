use std::{net::{TcpListener, TcpStream}, fmt::format, io::{BufReader, BufRead, Write, Read}};

use bc_protocal_lib::BuggersChatProtocalMessageType;

use crate::{fatalf, infof, warnf};

pub struct BuggersChatServer {
    addr: String,
}

impl BuggersChatServer {

    /// Initialize a new buggers chat server with the given address and port.
    pub fn new(address_and_port: &str) -> Self {
        Self {
            addr: String::from(address_and_port),
        }
    }

    /// Start the buggers chat server.
    pub fn start(&mut self) {
        let tcp_listener = match TcpListener::bind(&self.addr) {
            Ok(listener) => listener,
            Err(err) => {
                fatalf!("Cannot start server at {}: {}", self.addr, err);
            }
        };

        infof!("Server started at {:?}. ", tcp_listener.local_addr());

        for stream in tcp_listener.incoming() {
            if let Ok(stream) = stream {
                infof!("Connected to a stream: {:?}", stream);

                // Handle the connection.
                self.handle_connection(stream);
            }
        }
    }

    // Connect to a stream
    fn handle_connection(&mut self, mut stream: TcpStream) {

        // Read the content
        loop {
            let msg = if let Ok(some) = bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream) {
                some
            } else {
                BuggersChatProtocalMessageType::Disconnect
            };
            if let BuggersChatProtocalMessageType::String(content) = msg {
                infof!("Got request content: {:?}", content);
                if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, "nop") {}
                if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream) {};
            } else if let BuggersChatProtocalMessageType::Disconnect = msg {
                infof!("Disconnected. ");
                break;
            }
        }
    }

    
}

