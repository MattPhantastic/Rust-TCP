use crate::http::request::Request;
use crate::http::{
    Method,
    Endpoint,
    RequestParameter,
    Version,
    Header
};
use nom::bytes::complete::{take_until, take_while};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char},
    combinator::{map, opt, value},
    multi::{separated_list0},
    sequence::{preceded, separated_pair},
    IResult
};
use tracing::trace;

pub fn parse_http_request(input: &str) -> IResult<&str, Request> {
    trace!("Entering parse_http_request");
    let (input, (method, endpoint, version)) = parse_http_request_line(input)?;
    let (body, headers) = parse_http_headers(input)?;
    trace!("Exiting parse_http_request ({:?}, {:?}, {:?}, {:?}, {:?})", method, endpoint, version, headers, body);
    Ok((input, Request { method, endpoint, version, headers, body }))
}

pub fn parse_http_request_line(input: &str) -> IResult<&str, (Method, Endpoint, Version)> {
    trace!("Entering parse_http_request_line");
    let (input, method) = parse_http_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, endpoint) = parse_http_endpoint(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = parse_http_version(input)?;
    let (input, _) = tag("\r\n")(input)?;
    trace!("Exiting parse_http_request_line ({:?}, {:?}, {:?})", method, endpoint, version);
    Ok((input, (method, endpoint, version)))
}

pub fn parse_http_method(input: &str) -> IResult<&str, Method> {
    trace!("Entering parse_http_method");
    let (input, method) = alt((
        value(Method::GET, tag("GET")),
        value(Method::HEAD, tag("HEAD")),
        value(Method::POST, tag("POST")),
        value(Method::PUT, tag("PUT")),
        value(Method::DELETE, tag("DELETE")),
        value(Method::CONNECT, tag("CONNECT")),
        value(Method::OPTIONS, tag("OPTIONS")),
        value(Method::TRACE, tag("TRACE")),
        value(Method::PATCH, tag("PATCH")),
        map(alpha1, |s| Method::OTHER(s)),
    ))(input)?;
    trace!("Exiting parse_http_method ({:?})", method);
    Ok((input, method))
}

pub fn parse_http_endpoint(input: &str) -> IResult<&str, Endpoint> {
    trace!("Entering parse_http_endpoint");
    let (input, _) = char('/')(input)?;
    let (input, segments) = separated_list0(char('/'), take_while(valid_character))(input)?;
    let (input, parameters) = map(opt(parse_http_request_parameters),|a| a.unwrap_or(Vec::default()))(input)?;
    let (input, fragment) = opt(preceded(char('#'), alpha1))(input)?;
    trace!("Exiting parse_http_endpoint ({:?}, {:?}, {:?})", segments, parameters, fragment);
    Ok((input, Endpoint { segments, parameters, fragment }))
}

pub fn parse_http_request_parameters(input: &str) -> IResult<&str, Vec<RequestParameter>> {
    trace!("Entering parse_http_request_parameters");
    let (input, _) = char('?')(input)?;
    let (input, parameters) = separated_list0(char('&'), parse_http_request_parameter)(input)?;
    trace!("Exiting parse_http_request_parameters ({:?})", parameters);
    Ok((input, parameters))
}

pub fn parse_http_request_parameter(input: &str) -> IResult<&str, RequestParameter> {
    trace!("Entering parse_http_request_parameter");
    let (input, (name, value)) = separated_pair(
        take_while1(valid_character),
        char('='),
        take_while1(valid_character),
    )(input)?;
    trace!( "Exiting parse_http_request_parameter ({:?}, {:?})", name, value );
    Ok((input, RequestParameter { name, value }))
}

pub fn parse_http_version(input: &str) -> IResult<&str, Version> {
    trace!("Entering parse_http_version");
    let (input, version) = alt((
        value(Version::HTTP1_0, tag("HTTP/1.0")),
        value(Version::HTTP1_1, tag("HTTP/1.1")),
        map(alpha1, |s| Version::OTHER(s)),
    ))(input)?;
    trace!("Exiting parse_http_version");
    Ok((input, version))
}

fn parse_http_headers(input: &str) -> IResult<&str, Vec<Header>> {
    trace!("Entering parse_http_headers");
    let (input, headers) = separated_list0(tag("\r\n"), parse_http_header)(input)?;
    trace!("Exiting parse_http_headers");
    Ok((input, headers))
}

fn parse_http_header(input: &str) -> IResult<&str, Header> {
    trace!("Entering parse_http_header");
    let (input, (name, value)) =
        separated_pair(take_while1(valid_character), tag(": "), take_until("\r\n"))(input)?;
    trace!("Exiting parse_http_header ({:?}, {:?})", name, value);
    Ok((input, Header { name, value }))
}

fn valid_character(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z')
        || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9')
        || ch == '-'
        || ch == '_'
        || ch == '.'
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn parse_http_request_works() {
        let input =
            "GET /favicon.ico HTTP/1.1\r\n\
            Host: 127.0.0.1:8080\r\n\
            Connection: keep-alive\r\n\
            User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36\r\n\
            Accept: image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8\r\n\
            Sec-GPC: 1\
            Accept-Language: da\r\n\
            Sec-Fetch-Site: same-origin\r\n\
            Sec-Fetch-Mode: no-cors\r\n\
            Sec-Fetch-Dest: image\r\n\
            Referer: http://127.0.0.1:8080/\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            ";
            //"GET /some/service/path?hello=world&foo=bar#fragment HTTP/1.1\r\n\
            //User-Agent: curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3\r\n\
            //Host: www.example.com\r\n\
            //Accept-Language: en\r\n\
            //\r\n\
            //";
        match parse_http_request(input) {
            Ok(result) => {
                let request = result.1;
                println!("\n{:?}", request);
                assert_eq!(
                    request.method, Method::GET,
                    "The two methods we're comparing are not the same!"
                );
                assert_eq!(
                    request.endpoint.segments, vec!["some", "service", "path"],
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
                println!("\n{}", request);
            },
            _ => {}
        }
    }
}
