use crate::{Parse, Parser};
use std::ops::BitAnd;

#[derive(Copy, Clone, Debug)]
pub struct Concat<L, R>(L, R);

impl<'i, L, R> Parse<&'i str> for Concat<L, R>
    where
        L: Parse<&'i str, Out=String>,
        R: Parse<&'i str, Out=String, Err=L::Err>,
{
    type Err = L::Err;
    type Out = String;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let (mut l, rest) = self.0.parse(input)?;
        let (r, rest) = self.1.parse(rest)?;
        l.push_str(&r);

        Ok((l, rest))
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

        assert_eq!(p.parse("ab"), Ok(("ab".to_string(), "")));
        assert_eq!(p.parse("abc"), Ok(("ab".to_string(), "c")));

        let p = par("~") & "#_" & "." & "" & "$";

        assert_eq!(p.parse("~#_.$*"), Ok(("~#_.$".to_string(), "*")));
    }
}
