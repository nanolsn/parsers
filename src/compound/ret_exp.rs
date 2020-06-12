use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct RetExp<E>(pub E);

pub fn ret_exp<E>(value: E) -> RetExp<E>
    where
        E: Copy,
{ RetExp(value) }

impl<'r, I: 'r, E> Rule<'r, I> for RetExp<E>
    where
        E: Copy,
{
    type Mat = ();
    type Exp = E;

    fn rule(&'r self, _: I) -> Ruled<I, Self::Mat, Self::Exp> { Expected(self.0) }
}

impl_ops!(RetExp<V>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret_exp() {
        let r = super::ret_exp(12);
        assert_eq!(r.rule("hello!"), Expected(12));
    }
}
