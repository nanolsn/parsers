use crate::{Parse, Parser, Parsed};

pub struct Boxed<'p, I, O, E>(pub(crate) Box<dyn Parse<'p, Res=O, Err=E, On=I> + 'p>);

impl<'p, I, O, E> Parse<'p> for Boxed<'p, I, O, E>
    where
        I: 'p,
        O: 'p,
        E: 'p,
{
    type Res = O;
    type Err = E;
    type On = I;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input)
    }
}

pub type BoxedStrParser<'p, R, E=()> = Parser<Boxed<'p, &'p str, R, E>>;
pub type BoxedParser<'p, I, R, E=()> = Parser<Boxed<'p, I, R, E>>;
