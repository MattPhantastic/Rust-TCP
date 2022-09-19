pub mod request;

#[derive(Clone, Debug, PartialEq)]
struct Field<'a> {
    name: &'a str,
    value: &'a str
}
