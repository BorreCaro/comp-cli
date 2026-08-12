use std::error::Error;
use tiny_http::Response;

pub fn listen(server: &tiny_http::Server) -> Result<String, Box<dyn Error>> {
    let mut request = server.recv()?;
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content)?;
    request.respond(Response::empty(200))?;
    return Ok(content);
}
