use crate::{Parse, Parser, Parsed};
use std::ops::BitXor;

#[derive(Copy, Clone, Debug)]
pub struct RepeatVec<P>(pub(crate) P, pub(crate) usize);

impl<'p, P> Parse<'p> for RepeatVec<P>
    where
        P: Parse<'p>,
{
    type Res = Vec<P::Res>;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, mut rest: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        if self.1 == 0 {
            return Ok((vec![], rest));
        }

        let mut v = Vec::new();

        for _ in 0..self.1 {
            let (out, r) = self.0.parse(rest)?;
            rest = r;
            v.push(out);
        }
        Ok((v, rest))
    }
}

impl<P> BitXor<usize> for Parser<P> {
    type Output = Parser<RepeatVec<P>>;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Parser(RepeatVec(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use crate::{par, Parse};

    #[test]
    fn repeat() {
        let p = par('.') ^ 3;

        assert_eq!(p.parse("...."), Ok((vec![".", ".", "."], ".")));
        assert_eq!(p.parse("..."), Ok((vec![".", ".", "."], "")));
        assert_eq!(p.parse(".."), Err(()));

        let p = par('#') ^ 0;

        assert_eq!(p.parse("@"), Ok((vec![], "@")));
    }
}
