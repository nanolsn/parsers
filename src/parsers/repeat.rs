use crate::{Parse, Parser};
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Repeat<P>(pub(crate) P, pub(crate) usize);

impl<P, I> Parse<I> for Repeat<P>
    where
        P: Parse<I>,
{
    type Err = P::Err;
    type Out = Vec<P::Out>;

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
        let mut v = Vec::with_capacity(self.1);
        for _ in 0..self.1 {
            let (out, r) = self.0.parse(rest)?;
            rest = r;
            v.push(out);
        }
        Ok((v, rest))
    }
}

impl<P> Mul<usize> for Parser<P> {
    type Output = Parser<Repeat<P>>;

    fn mul(self, rhs: usize) -> Self::Output {
        Parser(Repeat(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use crate::{par, Parse};
    use std::iter::FromIterator;

    #[test]
    fn repeat() {
        let p = (par('.') * 3).map(|v| String::from_iter(v));

        assert_eq!(p.parse("...."), Ok(("...".to_string(), ".")));
        assert_eq!(p.parse("..."), Ok(("...".to_string(), "")));
        assert_eq!(p.parse(".."), Err(()));

        let p = par('#') * 0;

        assert_eq!(p.parse("@"), Ok((vec![], "@")));
    }
}
