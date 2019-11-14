use crate::Comply;
use crate::Parser;
use std::ops::Shr;
use crate::Rule;

#[derive(Copy, Clone, Debug)]
pub struct Second<A, B>(pub A, pub B);

impl<'p, A, B> Comply<'p> for Second<A, B>
    where
        A: Comply<'p>,
        B: Comply<'p, Err=A::Err, On=A::On>,
{
    type Res = B::Res;
    type Err = A::Err;
    type On = A::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let _ = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let b = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok(b)
    }
}

impl<A, B> Shr<B> for Rule<A> {
    type Output = Rule<Second<A, B>>;

    fn shr(self, rhs: B) -> Self::Output {
        Rule(Second(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn first() {
        let r = rule('0') >> '1';

        assert_eq!(
            Parser::new("01.").parse(r),
            (Ok("1"), "."),
        );
        assert_eq!(
            Parser::new("0!.").parse(r),
            (Err(()), "0!."),
        );
        assert_eq!(
            Parser::new("!1.").parse(r),
            (Err(()), "!1."),
        );
    }
}
