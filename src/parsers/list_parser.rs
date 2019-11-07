use crate::{Parse, Parser};
use std::ops::Rem;

#[derive(Copy, Clone, Debug)]
pub struct HeadParser<P>(P);

impl<P, I> Parse<I> for HeadParser<P>
    where
        P: Parse<I>,
{
    type Err = P::Err;
    type Out = Vec<P::Out>;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        let (out, rest) = self.0.parse(input)?;
        Ok((vec![out], rest))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ListParser<H, T>(H, T);

impl<H, T, I, J> Parse<I> for ListParser<H, T>
    where
        H: Parse<I, Out=Vec<J>>,
        T: Parse<I, Out=J, Err=H::Err>,
{
    type Err = H::Err;
    type Out = Vec<J>;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
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
        let e = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(ab.parse("abc"), Ok((e, "")));
    }
}
