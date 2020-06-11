use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

/// Match a whitespace character.
///
/// Match one of ` `, `\n`, `\r`, `\t`, `\r\n`.
///
/// # Examples
///
/// ```
/// # use parsers::{Apply, Ruled, Expected, common::white};
/// let rule = white();
///
/// let space = " ";
/// let nl = "\n";
/// let letter = "A";
///
/// assert_eq!(Ruled::Match(" ", ""), rule.apply(space));
/// assert_eq!(Ruled::Match("\n", ""), rule.apply(nl));
/// assert_eq!(Ruled::Expected(Expected::White), rule.apply(letter));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct White;

/// Constructor of [`White`]
///
/// [`White`]: ./struct.White.html
pub fn white() -> Rule<White> { Rule(White) }

impl<'i> Apply<&'i str> for White {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        let c = match input.chars().next() {
            Some(c @ ' ') => c,
            Some(c @ '\n') => c,
            Some(c @ '\r') => c,
            Some(c @ '\t') => c,
            _ => return Ruled::Expected(Expected::White),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::apply::apply;

    #[test]
    fn white() {
        assert!(apply(super::white(), " ").is_ok());
        assert!(apply(super::white(), "\n").is_ok());
        assert!(apply(super::white(), "\t").is_ok());
        assert!(apply(super::white(), "!").is_err());
    }
}
