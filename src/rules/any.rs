use crate::{Comply, Parser, Rule};

#[derive(Copy, Clone, Debug)]
pub struct Any;

impl<'p> Comply<'p> for Any {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            None => Err(()),
            Some(c) => Ok(parser.step(c.len_utf8())),
        }
    }
}

pub const fn any() -> Rule<Any> {
    Rule(Any)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any() {
        let any = super::any();

        assert_eq!(
            Parser::new("%^&").parse(any),
            (Ok("%"), "^&"),
        );
        assert_eq!(
            Parser::new("").parse(any),
            (Err(()), "")
        );
    }
}
