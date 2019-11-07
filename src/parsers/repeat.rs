use crate::{Parse, Parser};
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Repeat<P>(pub(crate) P, pub(crate) usize);

impl<P, I> Parse<I> for Repeat<P>
    where
        P: Parse<I, Out=String>,
{
    type Err = P::Err;
    type Out = String;

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
        if self.1 == 0 {
            return Ok((String::new(), rest));
        }

        let (mut s, r) = self.0.parse(rest)?;
        rest = r;

        for _ in 1..self.1 {
            let (out, r) = self.0.parse(rest)?;
            rest = r;
            s.push_str(&out);
        }
        Ok((s, rest))
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

    #[test]
    fn repeat() {
        let p = par('.').map_to_string() * 3;

        assert_eq!(p.parse("...."), Ok(("...".to_string(), ".")));
        assert_eq!(p.parse("..."), Ok(("...".to_string(), "")));
        assert_eq!(p.parse(".."), Err(()));

        let p = par('#').map_to_string() * 0;

        assert_eq!(p.parse("@"), Ok((String::new(), "@")));
    }
}
