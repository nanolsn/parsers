use crate::{Parse, Parser};
use std::ops::BitXor;

#[derive(Copy, Clone, Debug)]
pub struct RepeatConcat<P>(pub(crate) P, pub(crate) usize);

impl<'i, P> Parse<&'i str> for RepeatConcat<P>
    where
        P: Parse<&'i str, Out=&'i str>,
{
    type Err = P::Err;
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let mut len = 0;
        let mut rest = input;

        for _ in 0..self.1 {
            let (out, r) = self.0.parse(rest)?;
            rest = r;
            len += out.len();
        }

        assert!(len <= input.len());
        Ok((&input[..len], rest))
    }
}

impl<P> BitXor<usize> for Parser<P> {
    type Output = Parser<RepeatConcat<P>>;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Parser(RepeatConcat(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use crate::{par, Parse};

    #[test]
    fn repeat_concat() {
        let p = par('.') ^ 3;

        assert_eq!(p.parse(String::from("....").as_str()), Ok(("...", ".")));
        assert_eq!(p.parse("..."), Ok(("...", "")));
        assert_eq!(p.parse(".."), Err(()));

        let f = |c: char| c.is_digit(10);
        let p = par(f) ^ 2;

        assert_eq!(p.parse("01"), Ok(("01", "")));
        assert_eq!(p.parse("012"), Ok(("01", "2")));
        assert_eq!(p.parse("qwe"), Err(()));

        let p = par('#') ^ 0;

        assert_eq!(p.parse("@"), Ok(("", "@")));
    }
}
