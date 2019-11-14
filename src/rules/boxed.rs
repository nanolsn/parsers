use crate::{Rule, Comply, Parser};

pub type BoxedRule<'p, R, E=(), O=&'p str> = Rule<Box<dyn Comply<'p, Res=R, Err=E, On=O> + 'p>>;

impl<'p, R, E, O> Comply<'p> for Box<dyn Comply<'p, Res=R, Err=E, On=O> + 'p>
    where
        R: 'p,
        E: 'p,
        O: 'p,
{
    type Res = R;
    type Err = E;
    type On = O;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.as_ref().comply(parser)
    }
}

pub fn boxed<'p, R>(rule: R) -> BoxedRule<'p, R::Res, R::Err, R::On>
    where
        R: Comply<'p> + 'p,
{
    Rule(Box::new(rule))
}
