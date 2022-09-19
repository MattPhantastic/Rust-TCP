use crate::http::request::{
    HttpRequest,
    HttpRequestMethod,
    HttpRequestPath,
    HttpRequestParameter,
    HttpVersion,
    HttpHeader
};
use nom::bytes::complete::take_until;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, char, space1},
    combinator::{map, opt, value},
    multi::{many0, separated_list0},
    sequence::{preceded, separated_pair},
    IResult
};
use tracing::trace;

pub fn parse_http_request(input: &str) -> IResult<&str, HttpRequest> {
    trace!("Entering parse_http_request");
    let (input, (method, path, version)) = parse_http_request_line(input)?;
    let (input, headers) = parse_http_headers(input)?;
    trace!("Exiting parse_http_request ({:?}, {:?}, {:?}, {:?})", method, path, version, headers);
    Ok((input, HttpRequest { method, path, version, headers }))
}

pub fn parse_http_request_line(input: &str) -> IResult<&str, (HttpRequestMethod, HttpRequestPath, HttpVersion)> {
    trace!("Entering parse_http_request_line");
    let (input, method) = parse_http_request_method(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, path) = parse_http_request_path(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, version) = parse_http_version(input)?;
    let (input, _) = tag("\r\n")(input)?;
    trace!("Exiting parse_http_request_line ({:?}, {:?}, {:?})", method, path, version);
    Ok((input, (method, path, version)))
}

pub fn parse_http_request_method(input: &str) -> IResult<&str, HttpRequestMethod> {
    trace!("Entering parse_http_request_method");
    let (input, method) = alt((
        value(HttpRequestMethod::GET, tag("GET")),
        value(HttpRequestMethod::HEAD, tag("HEAD")),
        value(HttpRequestMethod::POST, tag("POST")),
        value(HttpRequestMethod::PUT, tag("PUT")),
        value(HttpRequestMethod::DELETE, tag("DELETE")),
        value(HttpRequestMethod::CONNECT, tag("CONNECT")),
        value(HttpRequestMethod::OPTIONS, tag("OPTIONS")),
        value(HttpRequestMethod::TRACE, tag("TRACE")),
        value(HttpRequestMethod::PATCH, tag("PATCH")),
        map(alpha1, |s| HttpRequestMethod::OTHER(s)),
    ))(input)?;
    trace!("Exiting parse_http_request_method ({:?})", method);
    Ok((input, method))
}

pub fn parse_http_request_path(input: &str) -> IResult<&str, HttpRequestPath> {
    trace!("Entering parse_http_request_path");
    let (input, _) = char('/')(input)?;
    let (input, segments) = separated_list0(char('/'), alpha1)(input)?;
    let (input, parameters) = map(opt(parse_http_request_parameters),|a| a.unwrap_or(Vec::default()))(input)?;
    let (input, fragment) = opt(preceded(char('#'), alpha1))(input)?;
    trace!("Exiting parse_http_request_path ({:?}, {:?}, {:?})", segments, parameters, fragment);
    Ok((input, HttpRequestPath { segments, parameters, fragment }))
}

pub fn parse_http_request_parameters(input: &str) -> IResult<&str, Vec<HttpRequestParameter>> {
    trace!("Entering parse_http_request_parameters");
    let (input, _) = char('?')(input)?;
    let (input, parameters) = separated_list0(char('&'), parse_http_request_parameter)(input)?;
    trace!("Exiting parse_http_request_parameters ({:?})", parameters);
    Ok((input, parameters))
}

pub fn parse_http_request_parameter(input: &str) -> IResult<&str, HttpRequestParameter> {
    trace!("Entering parse_http_request_parameter");
    let (input, (name, value)) = separated_pair(
        take_while1(is_header_character),
        char('='),
        take_while1(is_header_character),
    )(input)?;
    trace!( "Exiting parse_http_request_parameter ({:?}, {:?})", name, value );
    Ok((input, HttpRequestParameter { name, value }))
}

pub fn parse_http_version(input: &str) -> IResult<&str, HttpVersion> {
    trace!("Entering parse_http_version");
    let (input, version) = alt((
        value(HttpVersion::HTTP1_0, tag("HTTP/1.0")),
        value(HttpVersion::HTTP1_1, tag("HTTP/1.1")),
        map(alpha1, |s| HttpVersion::OTHER(s)),
    ))(input)?;
    trace!("Exiting parse_http_version");
    Ok((input, version))
}

fn parse_http_headers(input: &str) -> IResult<&str, Vec<HttpHeader>> {
    trace!("Entering parse_http_headers");
    let (input, headers) = separated_list0(tag("\r\n"), parse_http_header)(input)?;
    trace!("Exiting parse_http_headers");
    Ok((input, headers))
}

fn parse_http_header(input: &str) -> IResult<&str, HttpHeader> {
    trace!("Entering parse_http_header");
    let (input, (name, value)) =
        separated_pair(take_while1(is_header_character), tag(": "), take_until("\r\n"))(input)?;
    trace!("Exiting parse_http_header ({:?}, {:?})", name, value);
    Ok((input, HttpHeader { name, value }))
}

fn is_header_character(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z')
        || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9')
        || ch == '-'
        || ch == '_'
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
            "GET /path/to/entrypoint?hello=world&foo=bar#fragment HTTP/1.1\r
User-Agent: curl7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3\r
Host: www.example.com\r
Accept-Language: en\r
\r
";
        match parse_http_request(input) {
            Ok(result) => {
                let request = result.1;
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
