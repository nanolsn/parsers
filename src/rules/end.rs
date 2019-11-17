use crate::{Comply, Parser};

#[derive(Copy, Clone, Debug)]
pub struct End;

impl<'p> Comply<'p> for End {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        if parser.rest().is_empty() {
            Ok("")
        } else {
            Err(())
        }
    }
}

pub fn end() -> End {
    End
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn ret() {
        let r = rule('a') << super::end();

        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a"), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Err(()), "aa"),
        );
    }
}
