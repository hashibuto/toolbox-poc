use std::error;

pub fn handle_remote_http_session(endpoint: String) -> Result<(), Box<dyn error::Error>> {
    println!("remote session handled using http to endpoint: {}", endpoint);
    return Ok(())
}