use chatserver::BuggersChatServer;


mod chatserver;
mod utils;

fn main() {
    let mut server_obj = BuggersChatServer::new("0.0.0.0:8080");
    server_obj.start();
}
