use std::net::{Ipv4Addr};
use nom::character::complete::{char, digit1};
use nom::{IResult};
use nom::sequence::{preceded, tuple};

#[derive(Debug, PartialEq)]
struct Address {
    ipv4: Option<Ipv4Addr>,
    port: Option<u16>
}

fn parse_address(input: &str) -> IResult<&str, Address> {
    let (input, (a, _, b, _, c, _, d)) = tuple((digit1, char('.'), digit1, char('.'), digit1, char('.'), digit1))(input)?;
    let (input, port) = preceded(char(':'), digit1)(input)?;

    Ok((input, Address {
        ipv4: Option::from(Ipv4Addr::new(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap(), d.parse().unwrap())),
        port: Option::from(port.parse::<u16>().unwrap())
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
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
