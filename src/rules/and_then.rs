use crate::Comply;
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct AndThen<R, F>(pub R, pub F);

impl<'p, R, F, N> Comply<'p> for AndThen<R, F>
    where
        R: Comply<'p>,
        F: Fn(R::Res) -> N,
        N: Comply<'p, Err=R::Err, On=R::On> + 'p,
{
    type Res = N::Res;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        self.0.comply(parser)
            .map_err(|e| {
                assert_eq!(parser.get_pos(), pos);
                e
            })
            .and_then(|r| (self.1)(r).comply(parser))
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
    fn and_then() {
        fn then(s: &str) -> impl Comply<Res=&str, Err=(), On=&str> {
            rule('.') >> s
        }

        let r = (rule('0') | '1').and_then(then);

        assert_eq!(
            Parser::new("0.0.").parse(r),
            (Ok("0"), "."),
        );
        assert_eq!(
            Parser::new("1.1.").parse(r),
            (Ok("1"), "."),
        );
        assert_eq!(
            Parser::new("0.1.").parse(r),
            (Err(()), "0.1."),
        );
    }
}
