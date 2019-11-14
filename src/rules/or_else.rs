use crate::Comply;
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct OrElse<R, F>(pub R, pub F);

impl<'p, R, F, N> Comply<'p> for OrElse<R, F>
    where
        R: Comply<'p>,
        F: Fn(R::Err) -> N,
        N: Comply<'p, Res=R::Res, On=R::On>,
{
    type Res = R::Res;
    type Err = N::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        self.0.comply(parser)
            .or_else(|e| {
                assert_eq!(parser.get_pos(), pos);
                (self.1)(e).comply(parser)
            })
            .map_err(|e| {
                parser.set_pos(pos);
                e
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn or_else() {
        let r = rule('0').or_else(|_| rule('1'));

        assert_eq!(
            Parser::new("0.").parse(r),
            (Ok("0"), "."),
        );
        assert_eq!(
            Parser::new("1.").parse(r),
            (Ok("1"), "."),
        );
        assert_eq!(
            Parser::new("2.").parse(r),
            (Err(()), "2."),
        );
    }
}
