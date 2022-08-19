//! TODO: Make it better for user by using the crate reedline.

use std::{net::TcpStream, process::exit, io::{Write, stdout, stdin, ErrorKind}, thread, borrow::Cow};

use bc_protocal_lib::BuggersChatProtocalMessageType;
use crossbeam_channel::{unbounded, TryRecvError};
use crossterm::{style::Stylize, execute, terminal::{Clear, ClearType}, cursor, queue};
use json::object;
use reedline::{Reedline, Prompt, DefaultPrompt, EditMode, PromptEditMode, PromptViMode};

use crate::l10n;


#[derive(Clone)]
struct BuggersChatClientPrompt;

impl Prompt for BuggersChatClientPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        Cow::Owned(String::from("> "))
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        Cow::Owned(String::from(""))
    }

    fn render_prompt_indicator(&self, prompt_mode: reedline::PromptEditMode) -> std::borrow::Cow<str> {
        Cow::Owned(String::from(""))
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        Cow::Owned(String::from("... "))
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        Cow::Owned(String::from("=) "))
    }
}

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
        stream.set_nonblocking(true).unwrap();

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

        // TODO: Rewrite here
        let (tx, rx) = unbounded::<BuggersChatProtocalMessageType>();

        thread::spawn(move || loop {
            match bc_protocal_lib::BuggersChatProtocal::read_message(&mut stream) {
                Ok(msg) => {
                    if let BuggersChatProtocalMessageType::String(s) = msg {
                        if let Ok(json_obj) = json::parse(&s) {
                            if json_obj.has_key("type") {
                                queue!(stdout(), Clear(ClearType::CurrentLine),cursor::MoveToColumn(0)).ok();
                                match json_obj["type"].as_str().unwrap_or("") {
                                    "server_message" => {
                                        let content = l10n::get_string_by_language_and_key(crate::LANG, json_obj["localizable_id"].as_str().unwrap_or("str_unknown_msg_from_server")).replace("%USERNAME%", username.trim().clone());
                                        println!(
                                            "{}",
                                            format!("[SERVER] {content}").cyan().bold()
                                        );
                                    }
                                    "user_message" => {
                                        println!("{}", format!("[{}] {}", json_obj["from"].to_string(), json_obj["content"].to_string()));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    break;
                }
            }
            match rx.try_recv() {
                Ok(msg) => {
                    if let BuggersChatProtocalMessageType::String(s) = msg {
                        if let Ok(_) = bc_protocal_lib::BuggersChatProtocal::write_string(&mut stream, &object! {
                            "type": "send",
                            "content": s,
                            "username": username.clone().trim(),
                        }.dump()) {}
                    }
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break
            }
        });

        println!("{}", format!("[HINT] Type %quit% to quit.").bold().green());
        let mut line_editor = Reedline::create();
        let prompt = BuggersChatClientPrompt;
        loop {
            // stdin().read_line(&mut buffer).unwrap();
            let sig = line_editor.read_line(&prompt).unwrap();
            match sig {
                reedline::Signal::Success(buffer) => {
                    if buffer == "%quit%" {
                        break;
                    }
                    if let Err(err) = tx.send(BuggersChatProtocalMessageType::String(String::from(buffer.trim_end()))) {
                        println!("{err}")
                    }
                },
                reedline::Signal::CtrlC => {
                    break;
                }
                reedline::Signal::CtrlD => {}
            }
        }

    }
}
