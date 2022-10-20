use std::fmt::{Display, Formatter};

pub mod request;
pub mod response;

#[derive(Clone, Debug, PartialEq)]
pub struct Field<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: &'a str
}

impl<'a> Display for Field<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Method<'a> {
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

impl<'a> Default for Method<'a> {
    fn default() -> Self {
        Method::OTHER(&"")
    }
}

impl<'a> Display for Method<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::HEAD => write!(f, "HEAD"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::PATCH => write!(f, "PATCH"),
            Method::OTHER(method) => write!(f, "{}", method)
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Endpoint<'a> {
    pub segments: Vec<&'a str>,
    pub parameters: Vec<RequestParameter<'a>>,
    pub fragment: Option<&'a str>
}

pub type RequestParameter<'a> = Field<'a>;

#[derive(Clone, Debug, PartialEq)]
pub enum Version<'a> {
    HTTP1_0,
    HTTP1_1,
    OTHER(&'a str)
}

impl<'a> Default for Version<'a> {
    fn default() -> Self {
        Version::OTHER(&"")
    }
}

impl<'a> Display for Version<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::HTTP1_0 => write!(f, "HTTP 1.0"),
            Version::HTTP1_1 => write!(f, "HTTP 1.1"),
            Version::OTHER(protocol) => write!(f, "{}", protocol)
        }
    }
}

pub type Header<'a> = Field<'a>;
