use crate::Comply;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        let r = Not('a');

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
