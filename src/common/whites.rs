use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
    common::white,
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
/// # use parsers::{Apply, Ruled, Expected, common::whites};
/// let rule = whites();
///
/// let spaces = "  \t ";
/// let letter = "A";
///
/// assert_eq!(Ruled::Match("  \t ", ""), rule.apply(spaces));
/// assert_eq!(Ruled::Match("", "A"), rule.apply(letter));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Whites;

/// Constructor of [`Whites`]
///
/// [`Whites`]: ./struct.Whites.html
pub fn whites() -> Rule<Whites> { Rule(Whites) }

impl<'i> Apply<&'i str> for Whites {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        white().range(..).apply(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn whites() {
        assert_eq!(apply(super::whites(), " \n\rq"), Ruled::Match(" \n\r", "q"));
        assert_eq!(apply(super::whites(), "    q"), Ruled::Match("    ", "q"));
        assert_eq!(apply(super::whites(), "q"), Ruled::Match("", "q"));
    }
}
