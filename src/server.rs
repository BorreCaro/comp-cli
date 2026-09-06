use anyhow::Result;
use tiny_http::{Response, Server};

pub fn listen(server: &Server) -> Result<String> {
    let mut request = server.recv()?;
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content)?;
    request.respond(Response::empty(200))?;
    Ok(content)
}
