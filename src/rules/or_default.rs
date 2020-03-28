use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct OrDefault<R>(pub R);

impl<I, R> Apply<I> for OrDefault<R>
    where
        R: Apply<I>,
        I: Copy,
        R::Res: Default,
{
    type Err = R::Err;
    type Res = R::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.apply(input) {
            o @ Ruled::Ok(_, _) => o,
            Ruled::Err(_) => Ruled::Ok(Default::default(), input),
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
    fn or_default() {
        let r = rule("hello").or_default();
        assert_eq!(apply(&r, "hello"), Ruled::Ok("hello", ""));
        assert_eq!(apply(&r, "hi"), Ruled::Ok("", "hi"));
    }
}
