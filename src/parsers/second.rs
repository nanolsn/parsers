use crate::{Parse, Parser};
use std::ops::Shr;

#[derive(Copy, Clone, Debug)]
pub struct Second<L, R>(pub(crate) L, pub(crate) R);

impl<L, R, I> Parse<I> for Second<L, R>
    where
        L: Parse<I>,
        R: Parse<I, Err=L::Err>,
{
    type Err = L::Err;
    type Out = R::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).and_then(|(_, rest)| self.1.parse(rest))
    }
}

impl<L, R> Shr<R> for Parser<L> {
    type Output = Parser<Second<L, R>>;

    fn shr(self, rhs: R) -> Self::Output {
        Parser(Second(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn second() {
        let p = par('a') >> "b";

        assert_eq!(p.parse("ab_"), Ok(("b", "_")));
        assert_eq!(p.parse("wb"), Err(()));
    }
}
