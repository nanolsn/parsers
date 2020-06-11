use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

/// Match a whitespace character.
///
/// Match one of ` `, `\n`, `\r`, `\t`, `\r\n`.
///
/// # Examples
///
/// ```
/// # use parsers::{Rule, Ruled::*, SomeOf, basic::white};
/// let rule = white();
///
/// let space = " ";
/// let nl = "\n";
/// let letter = "A";
///
/// assert_eq!(Match(" ", ""), rule.rule(space));
/// assert_eq!(Match("\n", ""), rule.rule(nl));
/// assert_eq!(Expected(SomeOf::White), rule.rule(letter));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct White;

/// Constructor of [`White`]
///
/// [`White`]: ./struct.White.html
pub fn white() -> Rul<White> { Rul(White) }

impl<'i> Rule<&'i str> for White {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        let c = match input.chars().next() {
            Some(c @ ' ') => c,
            Some(c @ '\n') => c,
            Some(c @ '\r') => c,
            Some(c @ '\t') => c,
            _ => return Ruled::Expected(SomeOf::White),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white() {
        assert!(super::white().rule(" ").is_match());
        assert!(super::white().rule("\n").is_match());
        assert!(super::white().rule("\t").is_match());
        assert!(super::white().rule("!").is_expected());
    }
}
