use crate::{Parse, Parser};
use std::ops::BitAnd;

#[derive(Copy, Clone, Debug)]
pub struct Concat<L, R>(L, R);

impl<'i, L, R> Parse<&'i str> for Concat<L, R>
    where
        L: Parse<&'i str, Out=&'i str>,
        R: Parse<&'i str, Out=&'i str, Err=L::Err>,
{
    type Err = L::Err;
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let (l, rest) = self.0.parse(input)?;
        let (r, rest) = self.1.parse(rest)?;
        let len = l.len() + r.len();

        assert!(len <= input.len());
        Ok((&input[..len], rest))
    }
}

impl<L, R> BitAnd<R> for Parser<L> {
    type Output = Parser<Concat<L, R>>;

    fn bitand(self, rhs: R) -> Self::Output {
        Parser(Concat(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn concat() {
        let p = par('a') & "b";

        assert_eq!(p.parse("ab"), Ok(("ab", "")));
        assert_eq!(p.parse("abc"), Ok(("ab", "c")));

        let p = par("") & "" & "." & "";

        assert_eq!(p.parse(".*"), Ok((".", "*")));
    }
}
