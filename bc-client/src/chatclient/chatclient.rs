use std::{net::TcpStream, process::exit, io::{Write, stdout, stdin}, sync::{mpsc::channel, Arc, Mutex}};

use bc_protocal_lib::BuggersChatProtocalMessageType;
use crossterm::style::Stylize;
use json::object;

use crate::l10n;


pub struct BuggersChatClient {
    addr: String,
}

impl BuggersChatClient {
    /// Construct a new buggers chat client with given address.
    pub fn new(addr: &str) -> Self {
        Self { 
            addr: String::from(addr),
        }
    }

    /// Run the client.
    pub fn start(&mut self) {
        
        let mut stream = match TcpStream::connect(&self.addr) {
            Ok(s) => s,
            Err(err) => {
                eprintln!("{}", format!("{}{err}",l10n::get_string_by_language_and_key(crate::LANG, "str_fails_cnt")).bold().red());
                exit(-1);
            }
        };

        // Ask for the username.
        let mut username = String::new();
        print!("{}", l10n::get_string_by_language_and_key(crate::LANG, "str_ask_username"));
        stdout().flush().unwrap();
        stdin().read_line(&mut username).expect("Cannot read from stdin. Stopped.");

        // Login as the user.
        match bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
            "type": "login",
            "username": username.trim().clone(),
        }.dump()) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", format!("{}{err}",l10n::get_string_by_language_and_key(crate::LANG, "str_fails_login")).bold().red());
                exit(-1);
            }
        }
        if let Ok(_) = stream.flush() {}

        let (tx, rx) = channel::<String>();
        let (msg_sender, msg_receiver) = channel::<BuggersChatProtocalMessageType>();
        let arced_stream = Arc::new(Mutex::new(stream));
        let arced_stream_for_request_thread = arced_stream.clone();
        let arced_stream_for_guard_thread = arced_stream.clone();
        // The request thread.
        std::thread::spawn(move || {
            loop {
                if let Ok(mut stream) = arced_stream_for_request_thread.try_lock() {
                    if let Ok(msg) = bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream) {
                        if let Err(err) = msg_sender.send(msg) {
                            println!("{err}");
                        }
                    }
                }
            }
        });

        // The guard thread.
        std::thread::spawn(move || {
            loop {
                if let Ok(mut stream) = arced_stream_for_guard_thread.try_lock() {
                    // Get a command.
                    if let Ok(msg) = msg_receiver.try_recv() {
                        match msg {
                            BuggersChatProtocalMessageType::String(s) => {

                                // Read the object.
                                let obj = match json::parse(&s) {
                                    Ok(obj) => obj,
                                    Err(err) => {
                                        eprintln!("{}", format!("JSON Parse error: {}", err).bold().red());
                                        eprintln!("{}", format!("{}",l10n::get_string_by_language_and_key(crate::LANG, "str_tech_detail")).bold().cyan());
                                        exit(-1);
                                    }
                                };

                                // Check the value.
                                match obj["type"].to_string().as_str() {
                                    "server_message" => {
                                        println!("\n{}", format!(
                                            "{}",
                                            l10n::get_string_by_language_and_key(crate::LANG, obj["localizable_id"].to_string().as_str()).replace("%USERNAME%", username.trim())
                                        ).red().bold());
                                    }
                                    "user_message" => {
                                        println!("\n{}", format!(
                                            "[{}] {}",
                                            obj["from"].to_string(),
                                            obj["content"].to_string()
                                        ).green().bold());
                                    }
                                    _ => {}
                                }
                                if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream) {}
                            }
                            BuggersChatProtocalMessageType::Idle => {
                                // if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream) {}

                                // Trying to receive a message when idle.
                                match rx.try_recv() {
                                    Ok(some) => {
                                        if let Err(err) = bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
                                            "type": "send",
                                            "content": some.clone(),
                                        }.dump()) {
                                            eprintln!("{}", format!("{}{err}", l10n::get_string_by_language_and_key(crate::LANG, "str_failed_to_send")).bold().red());
                                        }
                                        stream.flush().unwrap();
                                    }
                                    Err(_) => {
                                        // Do something here.
                                        if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::make_idle(&mut stream) {}
                                    }
                                }
                            }
                            BuggersChatProtocalMessageType::Disconnect => {
                                eprintln!("{}", format!("{}",l10n::get_string_by_language_and_key(crate::LANG, "str_disconnected")).bold().cyan());
                                exit(0);
                            }
                        }
                    }
                }
            }
        });

        // Main loop.
        loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            if let Err(err) = tx.send(buffer) {
                eprintln!("{}", format!("{}({err})", l10n::get_string_by_language_and_key(crate::LANG, "str_disconnected")).bold().red());
                break;
            }
        }

    }
}
