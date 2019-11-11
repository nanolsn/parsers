use crate::{Parse, Parser, Parsed};
use std::ops::Shr;

#[derive(Copy, Clone, Debug)]
pub struct Second<L, R>(pub(crate) L, pub(crate) R);

impl<'p, L, R> Parse<'p> for Second<L, R>
    where
        L: Parse<'p>,
        R: Parse<'p, Err=L::Err, On=L::On>,
{
    type Res = R::Res;
    type Err = L::Err;
    type On = L::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
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
