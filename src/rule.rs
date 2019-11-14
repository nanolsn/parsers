use crate::Comply;
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct Rule<R>(pub R);

impl<'p, R> Comply<'p> for Rule<R>
    where
        R: Comply<'p>,
{
    type Res = R::Res;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser)
    }
}

pub fn rule<'p, R>(rule: R) -> Rule<R>
    where
        R: Comply<'p>,
{
    Rule(rule)
}
