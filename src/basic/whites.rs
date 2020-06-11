use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    SomeOf,
    basic::white,
};

/// Match any number of whitespace characters.
///
/// Applies [`White`] multiple times. This rule always returns [`Ruled::Ok`].
///
/// [`White`]: ./struct.White.html
/// [`Ruled::Ok`]: ../enum.Ruled.html#variant.Ok
///
/// # Examples
///
/// ```
/// # use parsers::{Rule, Ruled::*, SomeOf, basic::whites};
/// let rule = whites();
///
/// let spaces = "  \t ";
/// let letter = "A";
///
/// assert_eq!(Match("  \t ", ""), rule.rule(spaces));
/// assert_eq!(Match("", "A"), rule.rule(letter));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Whites;

/// Constructor of [`Whites`]
///
/// [`Whites`]: ./struct.Whites.html
pub fn whites() -> Rul<Whites> { Rul(Whites) }

impl<'i> Rule<&'i str> for Whites {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        white().range(..).rule(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whites() {
        assert_eq!(super::whites().rule(" \n\rq"), Ruled::Match(" \n\r", "q"));
        assert_eq!(super::whites().rule("    q"), Ruled::Match("    ", "q"));
        assert_eq!(super::whites().rule("q"), Ruled::Match("", "q"));
    }
}
