use std::{net::{TcpListener, TcpStream}, io::Write, thread, time::Duration};
use crossbeam_channel::unbounded as channel;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;

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
    pub fn start(&mut self, max_connection: i32) {
        let tcp_listener = match TcpListener::bind(&self.addr) {
            Ok(listener) => listener,
            Err(err) => {
                fatalf!("Cannot start server at {}: {}", self.addr, err);
            }
        };

        infof!("Server started at {:?}. ", tcp_listener.local_addr());

        // Main channel
        let (tx, rx) = channel::<String>();

        // Transfer/Receive pair for sending senders from new threads.
        let (ttx, rrx) = channel::<Sender<String>>();

        thread::spawn(move || {
            let mut msg_txs = Vec::<Sender<String>>::new();
            loop {
                if let Ok(data) = rx.try_recv() {
                    for i in &msg_txs {
                        infof!("data = {data}");
                        match i.send(data.clone()) {
                            Ok(_) => {}
                            Err(err) => {
                                warnf!("Fails to send message: {err}");
                            }
                        }
                    }
                }
                if let Ok(tx) = rrx.try_recv() {
                    infof!("Get a new channel sender: {tx:?}");
                    msg_txs.push(tx);
                }
            }
        });

        let mut connection_counter = 0;

        for stream in tcp_listener.incoming() {
            if let Ok(mut stream) = stream {
                infof!("Connected to a stream: {:?}", stream);
                if connection_counter >= max_connection {
                    infof!("But the connection limit has been reached... Disconnecting. ");
                    if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
                        "type": "server_message",
                        "localizable_id": "str_user_login",
                    }.to_string()) {}
                    if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::disconnect(&mut stream) {}
                }

                infof!("Creating channel sender for the stream...");
                let chan = tx.clone();

                infof!("Creating new server instance for the stream...");
                let mut server = self.clone();

                infof!("Creating new sender/receiver pair for the stream...");
                let (ntx, nrx) = channel::<String>();

                infof!("Creating thread for the stream...");
                // Handle the connection.
                thread::spawn(move || {
                    server.handle_connection(stream, chan, nrx); 
                });

                infof!("Sending ttx to the notification center...");
                // Send the ntx of new thread to the notification center.
                if let Ok(_) = ttx.send(ntx) {}
                infof!("Connection is successfully made.");
            }
        }
    }

    // Connect to a stream.
    fn handle_connection(&mut self, mut stream: TcpStream, chan: Sender<String>, notify: Receiver<String>) {

        // let mut user_name = String::from("<undefined>");
        let mut user_name;

        // Read the content
        loop {

            // Check the notification.
            if let Ok(data) = notify.recv_timeout(Duration::from_millis(5)) {
                match BuggersChatProtocal::write_string(&mut stream, &data) {
                    Ok(_) => {
                        infof!("Successfully receive a notification and send data. ");
                        if let Ok(_) = stream.flush() {}
                    }
                    Err(err) => {
                        warnf!("Error while sending data: {err}. Connection closed.");
                        if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::disconnect(&mut stream) {}
                        return;
                    }
                }
            }

            let msg = if let Ok(some) = bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream) {
                some
            } else {
                BuggersChatProtocalMessageType::Disconnect
            };
            if let BuggersChatProtocalMessageType::String(content) = msg {
                infof!("Got request content: {:?}", content);
                let json_obj = match json::parse(&content) {
                    Ok(obj) => obj,
                    Err(err) => {
                        warnf!("Error parsing JSON from the client: {}", err);
                        json::JsonValue::Null
                    }
                };
                // If parsing failed, then ignore it.
                if json_obj.is_null() {
                    continue;
                }
                // First connection.
                if json_obj.has_key("type") && json_obj["type"].is_string() && json_obj["type"].to_string() == "login" {
                    // Login.
                    user_name = json_obj["username"].to_string();
                    infof!("{0} logged in. ", user_name);
                    match chan.send(object! {
                        "type": "server_message",
                        "localizable_id": "str_user_login",
                        "username": user_name.clone(),
                    }.dump()) {
                        Ok(_) => {}
                        Err(err) => {
                            warnf!("Cannot send the message: {err} ");
                        }
                    }
                    continue;
                }
                // Send a message.
                if json_obj.has_key("type") && json_obj["type"].is_string() && json_obj["type"].to_string() == "send" {
                    // Get message content.
                    let message_content = json_obj["content"].to_string();
                    match chan.send(object! {
                        "type": "user_message",
                        "content": message_content,
                    }.dump()) {
                        Ok(_) => {}
                        Err(err) => {
                            warnf!("Cannot send the message: {err} ");
                        }
                    }
                    continue;
                }
            } else if let BuggersChatProtocalMessageType::Disconnect = msg {
                infof!("Disconnected. ");
                break;
            } else {
                // Idle
                if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream) {}
            }
        }
    }

    
}

