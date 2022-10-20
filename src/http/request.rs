use std::fmt::{Display, Formatter};
use crate::http::{Method, Endpoint, Version, Header};

pub mod parser;

#[derive(Clone, Debug, Default)]
pub struct Request<'a> {
    pub method: Method<'a>,
    pub endpoint: Endpoint<'a>,
    pub version: Version<'a>,
    pub headers: Vec<Header<'a>>,
    pub body: &'a str
}

impl<'a> TryFrom<&'a str> for Request<'a> {
    type Error = std::io::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parser::parse_http_request(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?
            .1)
    }
}

impl<'a> Display for Request<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![];
        for field in &self.endpoint.parameters {
            let param = field.name.to_string() + ": " + field.value;
            params.push(param);
        }
        let fragment;
        match self.endpoint.fragment {
            Some(f) => fragment = format!("#{}", f),
            None => fragment = "".parse().unwrap()
        }
        let mut headers = vec![];
        for field in &self.headers {
            let header = field.name.to_string() + ": " + field.value;
            headers.push(header);
        }
        write!(f,
               "\
               Request Method: {}\r\n  \
               Request Path: /{}\r\n    \
               Parameters: {}\r\n      \
               Fragment: {}\r\n  \
               HTTP version: {}\r\n  \
               HTTP Headers: {}",
               self.method,
               self.endpoint.segments.join("/"),
               params.join("\n                "),
               fragment,
               self.version,
               headers.join("\n                ")
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{RequestParameter};

    #[test]
    #[traced_test]
    fn try_from_and_display_works() {
        let input =
            "GET /path/to/entrypoint?hello=world&foo=bar#fragment HTTP/1.1\r\n\
            User-Agent: curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3\r\n\
            Host: www.example.com\r\n\
            Accept-Language: en\r\n
            \r\n\
            ";
        match Request::try_from(input) {
            Ok(request) => {
                println!("\n{:?}", request);
                assert_eq!(
                    request.method, Method::GET,
                    "The two methods we're comparing are not the same!"
                );
                assert_eq!(
                    request.endpoint.segments, vec!["path", "to", "entrypoint"],
                    "The two segment vectors we're comparing are not the same!"
                );
                assert_eq!(
                    request.endpoint.parameters, vec![
                        RequestParameter { name: "hello", value: "world" },
                        RequestParameter { name: "foo", value: "bar" }
                    ],
                    "The two parameter vectors we're comparing are not the same!"
                );
                assert_eq!(
                    request.endpoint.fragment, Option::from("fragment"),
                    "The two fragments we're comparing are not the same!"
                );
                assert_eq!(
                    request.version, Version::HTTP1_1,
                    "The two protocols we're comparing are not the same!"
                );
                assert_eq!(
                    request.headers, vec![
                        Header { name: "User-Agent", value: "curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3" },
                        Header { name: "Host", value: "www.example.com" },
                        Header { name: "Accept-Language", value: "en" }
                    ],
                    "The two header vectors we're comparing are not the same!"
                );
                println!("{}", request);
            }
            _ => {}
        }
    }
}
