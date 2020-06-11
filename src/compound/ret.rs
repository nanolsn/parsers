use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Ret<V>(pub V);

pub fn ret<V>(value: V) -> Rul<Ret<V>>
    where
        V: Copy,
{ Rul(Ret(value)) }

impl<I, V> Rule<I> for Ret<V>
    where
        V: Copy,
{
    type Exp = SomeOf<'static>;
    type Mat = V;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Match(self.0, input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret() {
        let r = super::ret(12);
        assert_eq!(r.rule("hello!"), Ruled::Match(12, "hello!"));
    }
}
