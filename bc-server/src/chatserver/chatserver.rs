//! TODO: Optimize it by using `tokio`.

use std::{net::TcpListener, io::ErrorKind, thread, time::Duration};
use crossbeam_channel::unbounded as channel;

use bc_protocal_lib::{BuggersChatProtocalMessageType, BuggersChatProtocal};
use json::*;

use crate::{fatalf, infof, warnf};

#[derive(Clone)]
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
    pub fn start(&mut self, _max_connection: i32) {
        let tcp_listener = match TcpListener::bind(&self.addr) {
            Ok(listener) => listener,
            Err(err) => {
                fatalf!("Cannot start server at {}: {}", self.addr, err);
            }
        };

        infof!("Server started at {:?}. ", tcp_listener.local_addr());

        if let Err(err) = tcp_listener.set_nonblocking(true) {
            fatalf!("Failed to initialize non-blocking: {err}");
        }

        // Clients.
        let mut clients = vec![];
        let (tx, rx) = channel::<BuggersChatProtocalMessageType>();

        loop {
            if let Ok((mut socket, addr)) = tcp_listener.accept() {
                infof!("Connecting with {addr}. ");
                let tx = tx.clone();
                clients.push(match socket.try_clone() {
                    Ok(s) => s,
                    Err(err) => {
                        warnf!("Cannot clone the socket: {err}. ");
                        continue;
                    }
                });

                // Spawn a thread to process the current client.
                thread::spawn(move || loop {
                    // Read a message.
                    match BuggersChatProtocal::read_message(&mut socket) {
                        Ok(msg) => {

                            // If no wrong with it, then send it out.
                            infof!("{addr}: {msg:?}");
                            match tx.send(msg) {
                                Ok(_) => {
                                    infof!("Successfully sent message to the main thread. ");
                                },
                                Err(err) => {
                                    warnf!("Error when sending message to another thread: {err}");
                                },
                            }
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(err) => {
                            warnf!("Failed to connect with {addr} due to: {err}");
                            break;
                        }
                    }

                    thread::sleep(Duration::from_millis(100));
                });

            }

            if let Ok(msg) = rx.try_recv() {
                infof!("Recieved message: {:?}", msg.clone());
                clients = clients.into_iter().filter_map(|mut client| {
                    match msg.clone() {
                        BuggersChatProtocalMessageType::String(msgt) => {
                            let obj = match json::parse(&msgt) {
                                Ok(obj) => obj,
                                Err(err) => {
                                    warnf!("Cannot parse the json from client({err}). Stopped.");
                                    return None;
                                }
                            };
                            // Process with obj here.
                            if obj.has_key("type") {
                                match obj["type"].as_str().unwrap_or("") {
                                    "send" => {
                                        if let Err(err) = BuggersChatProtocal::write_string(&mut client, &object! {
                                            "type": "user_message",
                                            "content": obj["content"].to_string(),
                                            "from": obj["username"].to_string(),
                                        }.dump()) {
                                            warnf!("Failed to send message: {err}");
                                        }
                                    },
                                    "login" => {
                                        if let Err(err) = BuggersChatProtocal::write_string(&mut client, &object! {
                                            "type": "server_message",
                                            "localizable_id": "str_user_login",
                                            "username": obj["username"].to_string(),
                                        }.dump()) {
                                            warnf!("Failed to send message: {err}");
                                        }
                                    },
                                    _ => {
                                        warnf!("Received a strange message from client: {obj}");
                                    }
                                }
                            }
                            return Some(client);
                        }
                        BuggersChatProtocalMessageType::Idle => {
                            return Some(client);
                        }
                        BuggersChatProtocalMessageType::Disconnect => {
                            return Some(client);
                        }
                    }
                }).collect::<Vec<_>>()
            }

            thread::sleep(Duration::from_millis(100));
        }

    }
    
}

