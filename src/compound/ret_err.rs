use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
};

#[derive(Copy, Clone, Debug)]
pub struct RetErr<E>(pub E);

pub fn ret_err<E>(value: E) -> Rul<RetErr<E>>
    where
        E: Copy,
{ Rul(RetErr(value)) }

impl<I, E> Rule<I> for RetErr<E>
    where
        E: Copy,
{
    type Exp = E;
    type Mat = ();

    fn rule(self, _: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Expected(self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ret_err() {
        let r = super::ret_err(12);
        assert_eq!(r.rule("hello!"), Ruled::Expected(12));
    }
}
