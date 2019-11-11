use crate::{Parse, Parser, Parsed};
use std::ops::BitAnd;

#[derive(Copy, Clone, Debug)]
pub struct Concat<L, R>(L, R);

impl<'p, L, R, S> Parse<'p> for Concat<L, R>
    where
        L: Parse<'p, Res=String, On=&'p str>,
        R: Parse<'p, Res=S, Err=L::Err, On=&'p str>,
        S: AsRef<str> + 'p,
{
    type Res = String;
    type Err = L::Err;
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let (mut l, rest) = self.0.parse(input)?;
        let (r, rest) = self.1.parse(rest)?;
        l.push_str(r.as_ref());

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
    use crate::stringed_par;

    #[test]
    fn concat() {
        let p = stringed_par('a') & "b";

        assert_eq!(p.parse("ab"), Ok(("ab".to_string(), "")));
        assert_eq!(p.parse("abc"), Ok(("ab".to_string(), "c")));

        let p = stringed_par("~") & "#_" & "." & "" & "$";

        assert_eq!(p.parse("~#_.$*"), Ok(("~#_.$".to_string(), "*")));
    }
}
