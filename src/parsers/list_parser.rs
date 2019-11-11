use crate::{Parse, Parser, Parsed};
use std::ops::Rem;

#[derive(Copy, Clone, Debug)]
pub struct HeadParser<P>(P);

impl<'p, P> Parse<'p> for HeadParser<P>
    where
        P: Parse<'p>,
{
    type Res = Vec<P::Res>;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let (out, rest) = self.0.parse(input)?;
        Ok((vec![out], rest))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ListParser<H, T>(H, T);

impl<'p, H, T, R> Parse<'p> for ListParser<H, T>
    where
        H: Parse<'p, Res=Vec<R>>,
        T: Parse<'p, Res=R, Err=H::Err, On=H::On>,
        R: 'p,
{
    type Res = Vec<R>;
    type Err = H::Err;
    type On = H::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let (mut h, rest) = self.0.parse(input)?;
        let (t, rest) = self.1.parse(rest)?;
        h.push(t);
        Ok((h, rest))
    }
}

impl<L, R> Rem<R> for Parser<L> {
    type Output = ListParser<HeadParser<L>, R>;

    fn rem(self, rhs: R) -> Self::Output {
        ListParser(HeadParser(self.0), rhs)
    }
}

impl<L, R, K> Rem<K> for ListParser<L, R> {
    type Output = ListParser<ListParser<L, R>, K>;

    fn rem(self, rhs: K) -> Self::Output {
        ListParser(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn test_list() {
        let ab = par("a") % 'b' % 'c';

        assert_eq!(ab.parse("abc"), Ok((vec!["a", "b", "c"], "")));
    }
}
