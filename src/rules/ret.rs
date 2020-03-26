use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Ret<V>(pub V);

pub fn ret<V>(value: V) -> Rule<Ret<V>>
    where
        V: Copy,
{ Rule(Ret(value)) }

impl<I, V> Apply<I> for Ret<V>
    where
        V: Copy,
{
    type Err = ();
    type Res = V;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Ok(self.0, input) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn ret() {
        let r = super::ret(12);
        assert_eq!(apply(r, "hello!"), Ruled::Ok(12, "hello!"));
    }
}
