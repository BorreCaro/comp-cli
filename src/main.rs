use crate::server::listen;
use tiny_http;
mod server;

fn main() {
    let Ok(server) = tiny_http::Server::http("127.0.0.1:27121") else {
        eprintln!("Couldn't lift server");
        return;
    };
    loop {
        let output = listen(&server);
        match output {
            Ok(output) => println!("{output}"),
            Err(e) => eprintln!("Error {e}"),
        }
    }
}
