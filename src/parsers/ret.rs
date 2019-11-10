use crate::{Parse, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Ret<T>(pub(crate) T);

pub fn ret<T>(val: T) -> Parser<Ret<T>>
    where
        T: Copy,
{
    Parser(Ret(val))
}

impl<T, I> Parse<I> for Ret<T>
    where
        T: Copy,
{
    type Err = ();
    type Out = T;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        Ok((self.0, input))
    }
}
