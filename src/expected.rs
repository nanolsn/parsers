#[derive(Debug, Eq, PartialEq)]
pub enum Expected<'r> {
    Char(char),
    Str(&'r str),
    String(String),
    AnyChar,
}
