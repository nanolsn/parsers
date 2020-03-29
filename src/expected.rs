#[derive(Debug, Eq, PartialEq)]
pub enum Expected<'r> {
    Char(char),
    Str(&'r str),
    String(String),
    CharRange(std::ops::Bound<char>, std::ops::Bound<char>),
    AnyChar,
    Bin,
    Oct,
    Dec,
    Hex,
    Latin,
    Nl,
    White,
}
