use crate::{Comply, Rule};
use crate::Parser;
use std::ops::BitOr;

#[derive(Copy, Clone, Debug)]
pub struct Or<A, B>(pub A, pub B);

impl<'p, A, B> Comply<'p> for Or<A, B>
    where
        A: Comply<'p>,
        B: Comply<'p, Res=A::Res, Err=A::Err, On=A::On>,
{
    type Res = A::Res;
    type Err = A::Err;
    type On = A::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        self.0.comply(parser).or_else(|_| {
            assert_eq!(parser.get_pos(), pos);
            self.1.comply(parser)
        })
    }
}

impl<'p, A, B> BitOr<B> for Rule<A>
    where
        A: Comply<'p>,
        B: Comply<'p, Res=A::Res, Err=A::Err, On=A::On>,
{
    type Output = Rule<Or<A, B>>;

    fn bitor(self, rhs: B) -> Self::Output {
        Rule(Or(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn or() {
        let r = Or(rule('@'), '#');

        assert_eq!(
            Parser::new("@".to_owned().as_str()).parse(r),
            (Ok("@"), ""),
        );
        assert_eq!(
            Parser::new("#").parse(r),
            (Ok("#"), ""),
        );
        assert_eq!(
            Parser::new("$").parse(r),
            (Err(()), "$"),
        );
    }
}
