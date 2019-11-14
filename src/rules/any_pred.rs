use crate::{Comply, Parser, Rule};

#[derive(Copy, Clone, Debug)]
pub struct AnyPred<F>(pub F);

impl<'p, F> Comply<'p> for AnyPred<F>
    where
        F: Fn(char) -> bool,
{
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c) if (self.0)(c) => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

pub fn any_pred<F>(f: F) -> Rule<AnyPred<F>>
    where
        F: Fn(char) -> bool,
{
    Rule(AnyPred(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_pred() {
        let r = super::any_pred(|s| s == '!');

        assert_eq!(
            Parser::new("!.").parse(r),
            (Ok("!"), "."),
        );
        assert_eq!(
            Parser::new("..").parse(r),
            (Err(()), ".."),
        );
        assert_eq!(
            Parser::new("").parse(r),
            (Err(()), ""),
        );
    }
}
