use crate::{Comply, Rule};
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct StringRes<P>(pub P);

impl<'p, P, S> Comply<'p> for StringRes<P>
    where
        P: Comply<'p, Res=S>,
        S: Into<String> + 'p,
{
    type Res = String;
    type Err = P::Err;
    type On = P::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser).map(|s| s.into())
    }
}

pub fn string_res<'p, R>(rule: R) -> Rule<StringRes<R>>
    where
        R: Comply<'p>,
{
    Rule(StringRes(rule))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn ret() {
        let r = StringRes(rule('@'));

        assert_eq!(
            Parser::new("@").parse(r),
            (Ok("@".to_string()), ""),
        );
    }
}
