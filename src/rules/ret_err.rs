use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct RetErr<E>(pub E);

pub fn ret_err<E>(value: E) -> Rule<RetErr<E>>
    where
        E: Copy,
{ Rule(RetErr(value)) }

impl<I, E> Apply<I> for RetErr<E>
    where
        E: Copy,
{
    type Err = E;
    type Res = ();

    fn apply(&self, _: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Err(self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn ret_err() {
        let r = super::ret_err(12);
        assert_eq!(apply(&r, "hello!"), Ruled::Err(12));
    }
}
