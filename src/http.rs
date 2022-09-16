use std::fmt::{Display, Formatter};

pub mod parser;

#[derive(Clone, Debug, Default)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod<'a>,
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

#[derive(Clone, Debug)]
pub enum HttpMethod<'a> {
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

impl<'a> Default for HttpMethod<'a> {
    fn default() -> Self {
        HttpMethod::OTHER(&"")
    }
}

impl<'a> Display for HttpMethod<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::TRACE => write!(f, "TRACE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::OTHER(method) => write!(f, "{}", method)
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HttpRequestPath<'a> {
    pub segments: Vec<&'a str>,
    pub query: Vec<QueryPair<'a>>,
    pub fragment: Option<&'a str>,
}

#[derive(Clone, Debug, Default)]
pub struct QueryPair<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Clone, Debug)]
pub enum HttpVersion<'a> {
    HTTP1_0,
    HTTP1_1,
    OTHER(&'a str),
}

impl<'a> Default for HttpVersion<'a> {
    fn default() -> Self {
        HttpVersion::OTHER(&"")
    }
}

#[derive(Clone, Debug, Default)]
pub struct HttpHeader<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    //use tracing_test::traced_test;

    #[test]
    //#[traced_test]
    fn it_works() {
        let input = r#"GET /path/to/file?hello=world&foo=bar#fragment HTTP/1.1
User-Agent: curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3
Host: www.example.com
Accept-Language: en, mi

        "#;
        let result = HttpRequest::try_from(input);
        println!("{:?}", result);
        //assert_eq!(result.method, HttpMethod::GET);
    }
}