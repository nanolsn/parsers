use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct RetErr<E>(pub E);

impl<I, E> Apply<I> for RetErr<E>
    where
        E: Copy,
{
    type Err = E;
    type Res = ();

    fn apply(&self, _: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Err(self.0) }
}

pub fn ret_err<E>(value: E) -> RetErr<E>
    where
        E: Copy,
{ RetErr(value) }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn ret_err() {
        let r = super::ret_err(12);
        assert_eq!(apply(r, "hello!"), Ruled::Err(12));
    }
}
