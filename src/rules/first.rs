use crate::Comply;
use crate::Parser;
use std::ops::Shl;
use crate::Rule;

#[derive(Copy, Clone, Debug)]
pub struct First<A, B>(pub A, pub B);

impl<'p, A, B> Comply<'p> for First<A, B>
    where
        A: Comply<'p>,
        B: Comply<'p, Err=A::Err, On=A::On>,
{
    type Res = A::Res;
    type Err = A::Err;
    type On = A::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let a = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let _ = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok(a)
    }
}

impl<A, B> Shl<B> for Rule<A> {
    type Output = Rule<First<A, B>>;

    fn shl(self, rhs: B) -> Self::Output {
        Rule(First(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn first() {
        let r = rule('0') << '1';

        assert_eq!(
            Parser::new("01.").parse(r),
            (Ok("0"), "."),
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
