use crate::{
    prelude::*,
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
/// # use parsers::{prelude::*, basic::whites};
/// assert!(whites().test("  \t "));
/// assert!(!whites().test("A"));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Whites;

/// Constructor of [`Whites`]
///
/// [`Whites`]: ./struct.Whites.html
pub fn whites() -> Whites { Whites }

impl<'r, 'i> Rule<'r, &'i str> for Whites {
    type Mat = &'i str;
    type Exp = Failed<'static>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        white().range(..).rule(input)
    }
}

impl_ops!(Whites);

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
