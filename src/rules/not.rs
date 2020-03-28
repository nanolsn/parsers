use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Not<A>(pub A);

impl<I, A> Apply<I> for Not<A>
    where
        A: Apply<I>,
        I: Copy,
{
    type Err = A::Res;
    type Res = A::Err;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.apply(input) {
            Ruled::Ok(r, _) => Ruled::Err(r),
            Ruled::Err(e) => Ruled::Ok(e, input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    #[test]
    fn not() {
        let r = !rule('a');
        assert_eq!(apply(&r, "a"), Ruled::Err("a"));
        assert_eq!(apply(&r, "b"), Ruled::Ok((), "b"));
    }
}
