use crate::{Parse, Parser};
use std::ops::Shl;

#[derive(Copy, Clone, Debug)]
pub struct First<L, R>(pub(crate) L, pub(crate) R);

impl<L, R, I> Parse<I> for First<L, R>
    where
        L: Parse<I>,
        R: Parse<I, Err=L::Err>,
{
    type Err = L::Err;
    type Out = L::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        let (out, rest) = self.0.parse(input)?;
        let (_, rest) = self.1.parse(rest)?;
        Ok((out, rest))
    }
}

impl<L, R> Shl<R> for Parser<L> {
    type Output = Parser<First<L, R>>;

    fn shl(self, rhs: R) -> Self::Output {
        Parser(First(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn first() {
        let p = par("a") << "b";

        assert_eq!(p.parse("ab_"), Ok(("a", "_")));
        assert_eq!(p.parse("wb"), Err(()));
    }
}
