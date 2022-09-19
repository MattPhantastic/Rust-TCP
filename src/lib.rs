use std::fmt::{Debug, Display, Formatter, write};
use std::io::{Error, ErrorKind, Read};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::ops::Add;
use std::str::FromStr;
use nom::character::complete::{char, digit1};
use nom::{IResult};
use nom::sequence::{preceded, tuple};
use crate::http::request::HttpRequest;

pub mod http;

pub fn listen(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let address = SocketAddr::from_str(addr)?;
    let listener = TcpListener::bind(address)?;

    for result in listener.incoming() {
        serve_client(result?); // your wish is my command!
    }
    Ok(())
}

fn serve_client(stream: TcpStream) {
    println!("Incoming request!");
    let mut buf = [0u8; 4096];
    match read_request(&stream, &mut buf) {
        Ok(request) => println!("{}", request),
        Err(e) => println!("{}", e)
    }
}

fn read_request<'a>(mut stream: &'a TcpStream, mut buf: &'a mut [u8]) -> Result<HttpRequest<'a>, Error> {
    let size = stream.read(&mut buf)?;
    HttpRequest::try_from(
        std::str::from_utf8(&buf[0..size])
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::{listen, serve_client};

    #[test]
    fn listen_works() {
        let address = "127.0.0.1:8080";
        match listen(address) {
            Ok(_) => {}
            Err(_) => {}
        }
        assert!(listen(address).is_ok(), "An error occurred when listening to {}", address);
        // it keeps listening... the fact that the test doesn't fail is a success.
    }
}
