use crate::{Parse, Parser, Parsed};
use std::ops::BitOr;

#[derive(Copy, Clone, Debug)]
pub struct OrParser<L, R>(pub(crate) L, pub(crate) R);

impl<'p, L, R> Parse<'p> for OrParser<L, R>
    where
        L: Parse<'p>,
        R: Parse<'p, Res=L::Res, Err=L::Err, On=L::On>,
        L::On: Copy,
{
    type Res = L::Res;
    type Err = L::Err;
    type On = L::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input).or_else(|_| self.1.parse(input))
    }
}

impl<L, R> BitOr<R> for Parser<L> {
    type Output = Parser<OrParser<L, R>>;

    fn bitor(self, rhs: R) -> Self::Output {
        Parser(OrParser(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn or_parser() {
        let p = par("a") | "b" | "c";

        assert_eq!(p.parse("a_"), Ok(("a", "_")));
        assert_eq!(p.parse("b_"), Ok(("b", "_")));
        assert_eq!(p.parse("c_"), Ok(("c", "_")));
        assert_eq!(p.parse("_"), Err(()));

        let p = par("") | "a";

        assert_eq!(p.parse("a"), Ok(("", "a")));
    }
}
