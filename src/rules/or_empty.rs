use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct OrEmpty<R>(pub R);

impl<'p, R> Comply<'p> for OrEmpty<R>
    where
        R: Comply<'p, Res=&'p str, On=&'p str>,
{
    type Res = &'p str;
    type Err = R::Err;
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        match self.0.comply(parser) {
            o @ Ok(_) => o,
            Err(_) => {
                assert_eq!(parser.get_pos(), pos);
                Ok("")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn or_empty() {
        let r = rule("hello").or_empty();

        assert_eq!(
            Parser::new("hello").parse(r),
            (Ok("hello"), ""),
        );
        assert_eq!(
            Parser::new("hi").parse(r),
            (Ok(""), "hi"),
        );
    }
}
