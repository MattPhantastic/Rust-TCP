use std::error::Error;
use std::io::{Read};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use nom::character::complete::{char, digit1};
use nom::{IResult};
use nom::sequence::{preceded, tuple};
use crate::http::HttpRequest;

mod http;

#[derive(Debug, PartialEq)]
struct Address {
    ipv4: Option<Ipv4Addr>,
    port: Option<u16>
}

impl Address {
    pub fn to_string(self) -> String {
        self.ipv4.unwrap().to_string() + ":" + &self.port.unwrap().to_string()
    }
}

fn parse_address(input: &str) -> IResult<&str, Address> {
    let (input, (a, _, b, _, c, _, d)) = tuple((digit1, char('.'), digit1, char('.'), digit1, char('.'), digit1))(input)?;
    let (input, port) = preceded(char(':'), digit1)(input)?;

    Ok((input, Address {
        ipv4: Option::from(Ipv4Addr::new(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap(), d.parse().unwrap())),
        port: Option::from(port.parse::<u16>().unwrap())
    }))
}

pub fn listen(addr: &str) {
    let result = parse_address(addr);
    match result {
        Ok(result) => {
            let addr = result.1;
            let listener = TcpListener::bind(addr.to_string()).unwrap();
            for result in listener.incoming() {
                match result {
                    Ok(stream) => serve_client(stream),  // your wish is my command!
                    _ => {}
                }
            }
        },
        _ => {}
    }
}

fn serve_client(stream: TcpStream) {
    println!("Incoming request!");
    read_request(&stream);
}

fn read_request(mut stream: &TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(size) => {
            let input = std::str::from_utf8(&buf[0..size])?;
            match HttpRequest::try_from(input) {
                Ok(request) => {
                    println!("Method: {}", request.method);
                },
                Err(e) => {}
            }
        },
        _ => {}
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, TcpStream};
    use std::str::FromStr;
    use super::{parse_address};

    #[test]
    fn parse_address_works() {
        let result = parse_address("127.0.0.1:8080");
        match result {
            Ok(addr) => {
                match addr.1.ipv4 {
                    Some(ipv4) => {
                        let expected = Ipv4Addr::from_str("127.0.0.1").unwrap();
                        assert_eq!(ipv4, expected,
                                   "The two IP addresses we're comparing are not the same!")
                    },
                    None => {}
                }
                match addr.1.port {
                    Some(port) => {
                        let expected = 8080;
                        assert_eq!(port, expected,
                                   "The two ports numbers we're comparing are not the same!")
                    },
                    None => {}
                }
            },
            _ => {}
        }
    }
}
