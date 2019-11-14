use crate::{Comply, Rule};
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct Not<A>(pub A);

impl<'p, A> Comply<'p> for Not<A>
    where
        A: Comply<'p>,
{
    type Res = A::Err;
    type Err = A::Res;
    type On = A::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        match self.0.comply(parser) {
            Ok(o) => {
                parser.set_pos(pos);
                Err(o)
            }
            Err(e) => Ok(e),
        }
    }
}

impl<'p, A> std::ops::Not for Rule<A>
    where
        A: Comply<'p>,
{
    type Output = Rule<Not<A>>;

    fn not(self) -> Self::Output {
        Rule(Not(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn not() {
        let r = !rule('a');

        assert_eq!(
            Parser::new("a").parse(r),
            (Err("a"), "a"),
        );
        assert_eq!(
            Parser::new("b").parse(r),
            (Ok(()), "b"),
        );
    }
}
