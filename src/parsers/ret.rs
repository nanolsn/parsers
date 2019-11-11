use crate::{Parse, Parser, Parsed};
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Ret<T, O>(pub(crate) T, pub(crate) PhantomData<O>);

pub fn ret<T, O>(val: T) -> Parser<Ret<T, O>>
    where
        T: Copy,
{
    Parser(Ret(val, PhantomData))
}

impl<'p, T, O> Parse<'p> for Ret<T, O>
    where
        T: Copy + 'p,
        O: 'p,
{
    type Res = T;
    type Err = ();
    type On = O;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        Ok((self.0, input))
    }
}
