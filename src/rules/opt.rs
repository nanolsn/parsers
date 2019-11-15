use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Opt<R>(pub R);

impl<'p, R> Comply<'p> for Opt<R>
    where
        R: Comply<'p, Res=&'p str, On=&'p str>,
{
    type Res = Option<R::Res>;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        match self.0.comply(parser) {
            Ok(o) => Ok(Some(o)),
            Err(_) => {
                assert_eq!(parser.get_pos(), pos);
                Ok(None)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn opt() {
        let r = rule("hello").opt();

        assert_eq!(
            Parser::new("hello").parse(r),
            (Ok(Some("hello")), ""),
        );
        assert_eq!(
            Parser::new("hi").parse(r),
            (Ok(None), "hi"),
        );
    }
}
