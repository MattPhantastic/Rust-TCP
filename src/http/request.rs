use std::fmt::{Display, Formatter};
use crate::http::Field;

pub mod parser;

#[derive(Clone, Debug, Default)]
pub struct HttpRequest<'a> {
    pub method: HttpRequestMethod<'a>,
    pub path: HttpRequestPath<'a>,
    pub version: HttpVersion<'a>,
    pub headers: Vec<HttpHeader<'a>>
}

impl<'a> TryFrom<&'a str> for HttpRequest<'a> {
    type Error = std::io::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parser::parse_http_request(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?
            .1)
    }
}

impl<'a> Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut params = vec![];
        for field in &self.path.parameters {
            let param = field.name.to_string() + ": " + field.value;
            params.push(param);
        }
        let fragment;
        match self.path.fragment {
            Some(f) => fragment = format!("#{}", f),
            None => fragment = "".parse().unwrap()
        }
        let mut headers = vec![];
        for field in &self.headers {
            let header = field.name.to_string() + ": " + field.value;
            headers.push(header);
        }
        write!(f,
               "Request Method: {}
  Request Path: /{}
    Parameters: {}
      Fragment: {}
  HTTP version: {}
  HTTP Headers: {}",
               self.method,
               self.path.segments.join("/"),
               params.join("\n                "),
               fragment,
               self.version,
               headers.join("\n                ")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HttpRequestMethod<'a> {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    OTHER(&'a str)
}

impl<'a> Default for HttpRequestMethod<'a> {
    fn default() -> Self {
        HttpRequestMethod::OTHER(&"")
    }
}

impl<'a> Display for HttpRequestMethod<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpRequestMethod::GET => write!(f, "GET"),
            HttpRequestMethod::HEAD => write!(f, "HEAD"),
            HttpRequestMethod::POST => write!(f, "POST"),
            HttpRequestMethod::PUT => write!(f, "PUT"),
            HttpRequestMethod::DELETE => write!(f, "DELETE"),
            HttpRequestMethod::CONNECT => write!(f, "CONNECT"),
            HttpRequestMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpRequestMethod::TRACE => write!(f, "TRACE"),
            HttpRequestMethod::PATCH => write!(f, "PATCH"),
            HttpRequestMethod::OTHER(method) => write!(f, "{}", method)
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HttpRequestPath<'a> {
    pub segments: Vec<&'a str>,
    pub parameters: Vec<HttpRequestParameter<'a>>,
    pub fragment: Option<&'a str>
}

pub type HttpRequestParameter<'a> = Field<'a>;

#[derive(Clone, Debug, PartialEq)]
pub enum HttpVersion<'a> {
    HTTP1_0,
    HTTP1_1,
    OTHER(&'a str)
}

impl<'a> Default for HttpVersion<'a> {
    fn default() -> Self {
        HttpVersion::OTHER(&"")
    }
}

impl<'a> Display for HttpVersion<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::HTTP1_0 => write!(f, "HTTP 1.0"),
            HttpVersion::HTTP1_1 => write!(f, "HTTP 1.1"),
            HttpVersion::OTHER(protocol) => write!(f, "{}", protocol)
        }
    }
}

pub type HttpHeader<'a> = Field<'a>;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn try_from_works() {
        let input =
            "GET /path/to/entrypoint?hello=world&foo=bar#fragment HTTP/1.1\r
User-Agent: curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3\r
Host: www.example.com\r
Accept-Language: en\r
\r
";
        match HttpRequest::try_from(input) {
            Ok(request) => {
                println!("\n{:?}", request);
                assert_eq!(
                    request.method, HttpRequestMethod::GET,
                    "The two methods we're comparing are not the same!"
                );
                assert_eq!(
                    request.path.segments, vec!["path", "to", "entrypoint"],
                    "The two segment vectors we're comparing are not the same!"
                );
                assert_eq!(
                    request.path.parameters, vec![
                        HttpRequestParameter { name: "hello", value: "world" },
                        HttpRequestParameter { name: "foo", value: "bar" }
                    ],
                    "The two parameter vectors we're comparing are not the same!"
                );
                assert_eq!(
                    request.path.fragment, Option::from("fragment"),
                    "The two fragments we're comparing are not the same!"
                );
                assert_eq!(
                    request.version, HttpVersion::HTTP1_1,
                    "The two protocols we're comparing are not the same!"
                );
                assert_eq!(
                    request.headers, vec![
                        HttpHeader { name: "User-Agent", value: "curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3" },
                        HttpHeader { name: "Host", value: "www.example.com" },
                        HttpHeader { name: "Accept-Language", value: "en" }
                    ],
                    "The two header vectors we're comparing are not the same!"
                );
                println!("\n{}", request);
            },
            _ => {}
        }
    }
}
