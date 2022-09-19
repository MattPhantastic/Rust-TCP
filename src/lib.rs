use std::fmt::{Debug, Display, Formatter, write};
use std::io::{Error, ErrorKind, Read};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::ops::Add;
use nom::character::complete::{char, digit1};
use nom::{IResult};
use nom::sequence::{preceded, tuple};
use crate::http::request::HttpRequest;

pub mod http;

#[derive(Debug, PartialEq)]
struct Address {
    ipv4: Ipv4Addr,
    port: u16
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ipv4.to_string(), self.port.to_string())
    }
}

fn parse_address(input: &str) -> IResult<&str, Address> {
    let (input, (a, _, b, _, c, _, d)) = tuple((digit1, char('.'), digit1, char('.'), digit1, char('.'), digit1))(input)?;
    let (input, port) = preceded(char(':'), digit1)(input)?;
    Ok((input, Address {
        ipv4: Ipv4Addr::new(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap(), d.parse().unwrap()),
        port: port.parse::<u16>().unwrap()
    }))
}

pub fn listen(addr: &str) {
    match parse_address(addr) {
        Ok(result) => {
            let address = result.1;
            let listener = TcpListener::bind(address.to_string()).unwrap();
            for result in listener.incoming() {
                match result {
                    Ok(stream) => serve_client(stream),  // your wish is my command!
                    Err(e) => {}
                }
            }
        },
        Err(e) => {}
    }
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
    use std::net::{Ipv4Addr, TcpStream};
    use std::str::FromStr;
    use super::{parse_address};

    #[test]
    fn parse_address_works() {
        match parse_address("127.0.0.1:8080") {
            Ok(result) => {
                let address = result.1;
                assert_eq!(
                    address.ipv4, Ipv4Addr::from_str("127.0.0.1").unwrap(),
                    "The two IP addresses we're comparing are not the same!"
                );
                assert_eq!(
                    address.port, 8080,
                    "The two ports numbers we're comparing are not the same!"
                );
                println!("\n{}", address);
            },
            _ => {}
        }
    }
}
