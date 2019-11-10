use crate::{Parse, Parser};

pub struct Boxed<'p, I, O, E>(pub(crate) Box<dyn Parse<I, Out=O, Err=E> + 'p>);

impl<'p, I, O, E> Parse<I> for Boxed<'p, I, O, E> {
    type Err = E;
    type Out = O;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input)
    }
}

pub type BoxedStrParser<'p, 'i, R, E=()> = Parser<Boxed<'p, &'i str, R, E>>;
pub type BoxedParser<'p, I, R, E=()> = Parser<Boxed<'p, I, R, E>>;
