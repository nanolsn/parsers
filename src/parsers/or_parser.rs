use crate::{Parse, Parser};
use std::ops::BitOr;

#[derive(Copy, Clone, Debug)]
pub struct OrParser<L, R>(pub(crate) L, pub(crate) R);

impl<L, R, I> Parse<I> for OrParser<L, R>
    where
        L: Parse<I>,
        R: Parse<I, Out=L::Out, Err=L::Err>,
        I: Copy,
{
    type Err = L::Err;
    type Out = L::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
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
