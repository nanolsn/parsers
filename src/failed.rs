/// The general type for the aggregation of failed matches.
///
/// Most of [`Rule`] implementations return a value of this type as a matching error.
/// This type allows you to collect different errors into the one,
/// making it possible to combine various [rules] together.
///
/// [`Rule`]: ./trait.Rule.html
/// [rules]: ./trait.Rule.html
#[derive(Debug, Eq, PartialEq)]
pub enum Failed<'r> {
    /// The `char` value.
    Char(char),

    /// The string reference.
    Str(&'r str),

    /// The char range.
    CharRange(std::ops::Bound<char>, std::ops::Bound<char>),

    /// Predicate checked.
    Predicate,

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

    /// Nothing.
    Nothing,
}

impl From<char> for Failed<'_> {
    fn from(c: char) -> Self { Failed::Char(c) }
}

impl<'r> From<&'r str> for Failed<'r> {
    fn from(s: &'r str) -> Self { Failed::Str(s) }
}

impl From<()> for Failed<'_> {
    fn from(_: ()) -> Self { Failed::Nothing }
}
