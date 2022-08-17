//! Currently test codes.

use std::io::{Write, stdin, stdout};


mod chatclient;
mod l10n;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static LANG: &'static str = "en_US";

fn main() {

    println!("{}{}", l10n::get_string_by_language_and_key(LANG, "str_greeting"), VERSION);
    print!("{}", l10n::get_string_by_language_and_key(LANG, "str_ask_addr"));
    let mut addr = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut addr).expect("Unable to read from stdin.");

    let mut client = chatclient::BuggersChatClient::new(addr.trim());
    client.start();


}
