use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Copy, Clone, Debug)]
pub struct Cat<A, B>(pub A, pub B);

impl<I, A, B, L, R> Apply<I> for Cat<A, B>
    where
        A: Apply<I, Res=L>,
        B: Apply<I, Res=R, Err=A::Err>,
        L: Concat<R>,
{
    type Err = A::Err;
    type Res = L::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|l, i| self.1.apply(i).map(|r| l.concat(r)))
    }
}
