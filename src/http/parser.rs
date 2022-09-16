use crate::http::{
    HttpRequest,
    HttpMethod,
    HttpRequestPath,
    HttpVersion,
    HttpHeader,
    QueryPair
};
use nom::bytes::complete::take_until;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, space1},
    combinator::{map, opt, value},
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn parse_http_request(input: &str) -> IResult<&str, HttpRequest> {
    //trace!("Entering parse_http_request");
    let (input, (method, path, version)) = parse_http_request_line(input)?;
    let (input, headers) = parse_http_headers(input)?;
    /*trace!(
        "Exiting parse_http_request ({:?}, {:?}, {:?}, {:?})",
        method,
        path,
        version,
        headers
    );*/
    Ok((
        input,
        HttpRequest {
            method,
            path,
            version,
            headers,
        },
    ))
}

pub fn parse_http_request_line(input: &str) -> IResult<&str, (HttpMethod, HttpRequestPath, HttpVersion)> {
    //trace!("Entering parse_http_request_line");
    let (input, method) = parse_http_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, path) = parse_http_path(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = parse_http_version(input)?;
    let (input, _) = tag("\n")(input)?;
    /*trace!(
        "Exiting parse_http_request_line ({:?}, {:?}, {:?})",
        method,
        path,
        version
    );*/
    Ok((input, (method, path, version)))
}

pub fn parse_http_method(input: &str) -> IResult<&str, HttpMethod> {
    //trace!("Entering parse_http_method");
    let (input, method) = alt((
        value(HttpMethod::GET, tag("GET")),
        value(HttpMethod::HEAD, tag("HEAD")),
        value(HttpMethod::POST, tag("POST")),
        value(HttpMethod::PUT, tag("PUT")),
        value(HttpMethod::DELETE, tag("DELETE")),
        value(HttpMethod::CONNECT, tag("CONNECT")),
        value(HttpMethod::OPTIONS, tag("OPTIONS")),
        value(HttpMethod::TRACE, tag("TRACE")),
        value(HttpMethod::PATCH, tag("PATCH")),
        map(alpha1, |s| HttpMethod::OTHER(s)),
    ))(input)?;
    //trace!("Exiting parse_http_method ({:?})", method);
    Ok((input, method))
}

pub fn parse_http_path(input: &str) -> IResult<&str, HttpRequestPath> {
    //trace!("Entering parse_http_path");
    let (input, _) = char('/')(input)?;
    let (input, segments) = separated_list0(char('/'), alpha1)(input)?;
    let (input, query) = map(opt(parse_http_path_query), |a| a.unwrap_or(Vec::default()))(input)?;
    let (input, fragment) = opt(preceded(char('#'), alpha1))(input)?;
    /*trace!(
        "Exiting parse_http_path ({:?}, {:?}, {:?})",
        segments,
        query,
        fragment
    );*/
    Ok((
        input,
        HttpRequestPath {
            segments,
            query,
            fragment,
        },
    ))
}

pub fn parse_http_path_query(input: &str) -> IResult<&str, Vec<QueryPair>> {
    //trace!("Entering parse_http_path_query");
    let (input, _) = char('?')(input)?;
    let (input, pairs) = separated_list0(char('&'), parse_http_path_query_pair)(input)?;
    //trace!("Exiting parse_http_path_query ({:?})", pairs);
    Ok((input, pairs))
}

pub fn parse_http_path_query_pair(input: &str) -> IResult<&str, QueryPair> {
    //trace!("Entering parse_http_path_query_pair");
    let (input, (key, value)) = separated_pair(
        take_while1(is_header_character),
        char('='),
        take_while1(is_header_character),
    )(input)?;
    /*trace!(
        "Exiting parse_http_path_query_pair ({:?}, {:?})",
        key,
        value
    );*/
    Ok((input, QueryPair { key, value }))
}

pub fn parse_http_version(input: &str) -> IResult<&str, HttpVersion> {
    //trace!("Entering parse_http_version");
    let (input, version) = alt((
        value(HttpVersion::HTTP1_0, tag("HTTP/1.0")),
        value(HttpVersion::HTTP1_1, tag("HTTP/1.1")),
        map(alpha1, |s| HttpVersion::OTHER(s)),
    ))(input)?;
    //trace!("Exiting parse_http_version");
    Ok((input, version))
}

fn parse_http_headers(input: &str) -> IResult<&str, Vec<HttpHeader>> {
    //trace!("Entering parse_http_headers");
    let (input, headers) = separated_list0(char('\n'), parse_http_header)(input)?;
    //trace!("Exiting parse_http_headers");
    Ok((input, headers))
}

fn parse_http_header(input: &str) -> IResult<&str, HttpHeader> {
    //trace!("Entering parse_http_header");
    let (input, (name, value)) =
        separated_pair(take_while1(is_header_character), tag(":"), take_until("\n"))(input)?;
    //trace!("Exiting parse_http_header ({:?}, {:?})", key, value);
    Ok((input, HttpHeader { name, value }))
}

fn is_header_character(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z')
        || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9')
        || ch == '-'
        || ch == '_'
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
        let result = parse_http_request(input);
        println!("{:?}", result.unwrap().1);
        //assert_eq!(result.method, HttpMethod::GET);
    }
}
