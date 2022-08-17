//! Currently test codes.

use std::{net::TcpStream, io::{Write}};

use bc_protocal_lib::BuggersChatProtocalMessageType;
use json::object;

mod webutils;

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    loop {
        bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
            "type": "login",
            "username": "27Onion",
        }.dump()).unwrap();
        stream.flush().unwrap();
        let msg = bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream).unwrap();
        if let BuggersChatProtocalMessageType::String(something) = msg {
            println!("{}", something);
            bc_protocal_lib::BuggersChatProtocal::disconnect(&mut stream).unwrap();
            break;
        } else if let BuggersChatProtocalMessageType::Disconnect = msg {
            println!("BREAK");
            break;
        }
    }   
}
