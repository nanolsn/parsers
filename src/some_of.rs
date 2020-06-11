#[derive(Debug, Eq, PartialEq)]
pub enum SomeOf<'r> {
    /// The certain `char` value.
    Char(char),

    /// The string reference.
    Str(&'r str),

    /// The char range.
    CharRange(std::ops::Bound<char>, std::ops::Bound<char>),

    /// Any char.
    AnyChar,

    /// The binary char (`0` or `1`).
    Bin,

    /// The octal char (`0..=7`).
    Oct,

    /// The decimal char (`0..=9`).
    Dec,

    /// The hexadecimal char (`0..=9` or `a..=f` or `A..=F`).
    Hex,

    /// Any latin char.
    Latin,

    /// The new line.
    Nl,

    /// Any whitespace char.
    White,
}
