use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use crate::http::{Header, Version};
use crate::http::request::Request;
use crate::http::response::{Response, StatusCode};

pub mod http;

struct Server {
    address: SocketAddr,
}

impl Server {
    pub fn new(addr: &str) -> std::io::Result<Server> {
        let address = SocketAddr::from_str(addr)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;
        Ok(Server { address })
    }

    pub fn serve(&self) -> std::io::Result<()> {
        for result in TcpListener::bind(self.address)?.incoming() {
            let mut stream = result?;
            let mut buffer = [0u8; 4096];
            let request = deserialize(&mut stream, &mut buffer)?;
            let response = route(&request);
            response.serialize(&mut stream)?;
        }
        Ok(())
    }

    fn deserialize<'a, T: std::io::Read>(stream: &mut T, mut buf: &'a mut [u8]) -> std::io::Result<Request<'a>> {
        let size = stream.read(&mut buf)?;
        println!("{}", std::str::from_utf8(&buf[0..size]).unwrap());
        Request::try_from(
            std::str::from_utf8(&buf[0..size])
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?
        )
    }

    fn route<'a>(_request: &Request<'a>) -> Response<'a> {
        Response {
            version: Version::HTTP1_1,
            status_code: StatusCode::Ok,
            headers: vec![
                Header {
                    name: "Content-type",
                    value: "text/html"
                }
            ],
            body: "<h2>Hello, world!</h2>"
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use crate::{Request, Version, Header, deserialize, serve};
    use crate::http::request::{Method, Endpoint};

    #[test]
    fn deserialize_works<'a>() {
        let mut stream: TcpStream;
        //stream.write("")
        let mut stream =
            "GET /index.html HTTP/1.1\r\n\
            Host: 127.0.0.1:8080\r\n\
            Connection: keep-alive\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36\r\n\
            Accept: image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8\r\n\
            Sec-GPC: 1\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Accept-Language: da\r\n\
            \r\n\
            ";
        let mut buf = [0u8; 4096];
        assert_eq!(deserialize(&mut stream, &mut buf), Ok(
            Request {
                method: Method::GET,
                endpoint: Endpoint {
                    segments: vec!["index.html"],
                    parameters: vec![],
                    fragment: None
                },
                version: Version::HTTP1_1,
                headers: vec![
                    Header {
                        name: "Host",
                        value: "127.0.0.1:8080"
                    },
                    Header {
                        name: "Connection",
                        value: "keep-alive"
                    },
                    Header {
                        name: "User-Agent",
                        value: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36"
                    },
                    Header {
                        name: "Accept",
                        value: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"
                    },
                    Header {
                        name: "Sec-GPC",
                        value: "1"
                    },
                    Header {
                        name: "Accept-Encoding",
                        value: "gzip, deflate, br"
                    },
                    Header {
                        name: "Accept-Language",
                        value: "da"
                    },
                ],
                body: ""
            }
        ))
    }
}
