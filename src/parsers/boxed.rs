use crate::Parse;

pub struct Boxed<I, O, E>(pub(crate) Box<dyn Parse<I, Out=O, Err=E>>);

impl<I, O, E> Parse<I> for Boxed<I, O, E> {
    type Err = E;
    type Out = O;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input)
    }
}
