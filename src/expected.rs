/// An enum for provision of information about an expected values in the [rule].
///
/// [rule]: ./trait.Apply.html
#[derive(Debug, Eq, PartialEq)]
pub enum Expected<'r> {
    /// Expected the certain `char` value.
    Char(char),

    /// Expected the string reference.
    Str(&'r str),

    /// Expected the certain `String` value.
    String(String),

    /// Expected the char range.
    CharRange(std::ops::Bound<char>, std::ops::Bound<char>),

    /// Expected any char.
    AnyChar,

    /// Expected the binary char (`0` or `1`).
    Bin,

    /// Expected the octal char (`0..=7`).
    Oct,

    /// Expected the decimal char (`0..=9`).
    Dec,

    /// Expected the hexadecimal char ('0..=9' or 'a..=f' (any case)).
    Hex,

    /// Expected any latin char.
    Latin,

    /// Expected the new line.
    Nl,

    /// Expected any whitespace char.
    White,
}
