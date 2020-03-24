use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Or<A, B>(pub A, pub B);

impl<I, A, B> Apply<I> for Or<A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
        I: Copy,
        A::Res: Into<B::Res>,
{
    type Err = A::Err;
    type Res = B::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .map(|l| l.into())
            .or_else(|_| self.1.apply(input))
    }
}
