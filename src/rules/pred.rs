use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Pred<R, F>(pub R, pub F);

impl<'p, R, F> Comply<'p> for Pred<R, F>
    where
        R: Comply<'p>,
        F: Fn(&R::Res) -> bool,
        R::Res: 'p,
{
    type Res = R::Res;
    type Err = ();
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        match self.0.comply(parser) {
            Ok(r) => {
                if (self.1)(&r) {
                    Ok(r)
                } else {
                    parser.set_pos(pos);
                    Err(())
                }
            }
            Err(_) => {
                assert_eq!(parser.get_pos(), pos);
                Err(())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rule, Or, Parser};

    #[test]
    fn pred() {
        let r = Pred(Or(rule("@"), "#"), |s: &&str| *s == "@");

        assert_eq!(
            Parser::new("@").parse(r),
            (Ok("@"), ""),
        );
        assert_eq!(
            Parser::new("#").parse(r),
            (Err(()), "#"),
        );
    }
}
