//! Currently test codes.

use std::{net::TcpStream, io::{Write, stdin}, time::Duration};
use crossbeam_channel::{ Receiver, Sender };
use crossbeam_channel::unbounded as channel;

use bc_protocal_lib::BuggersChatProtocalMessageType;
use json::object;

mod webutils;

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
        "type": "login",
        "username": "27Onion",
    }.dump()).unwrap();
    stream.flush().unwrap();
    let (tx, rx) = channel::<String>();
    std::thread::spawn(move || {
            loop {
                let msg = bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream).unwrap();
                if let BuggersChatProtocalMessageType::String(something) = msg {
                    println!("{}", something);
                } else if let BuggersChatProtocalMessageType::Disconnect = msg {
                    println!("BREAK");
                    break;
                }
                match rx.try_recv() {
                    Ok(some) => {
                        println!("Sending {some}");
                        bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
                            "type": "send",
                            "content": some.clone(),
                        }.dump()).unwrap();
                        println!("Sent {some}");
                        stream.flush().unwrap();
                    }
                    Err(err) => {
                        // println!("{err}");
                    }
                }
                bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream).unwrap();
            }  
        } 
    );
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        match tx.send(buffer) {
            Ok(_) => {
                println!("Sent");
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
}
