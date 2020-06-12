use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Ret<V>(pub V);

pub fn ret<V>(value: V) -> Ret<V>
    where
        V: Copy,
{ Ret(value) }

impl<'r, I: 'r, V> Rule<'r, I> for Ret<V>
    where
        V: Copy,
{
    type Mat = V;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { Match(self.0, input) }
}

impl_ops!(Ret<V>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret() {
        let r = super::ret(12);
        assert_eq!(r.rule("hello!"), Match(12, "hello!"));
    }
}
