use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Or<A, B>(pub A, pub B);

impl<I, A, B> Apply<I> for Or<A, B>
    where
        A: Apply<I>,
        B: Apply<I, Err=A::Err>,
        I: Copy,
        A::Res: Into<B::Res>,
{
    type Err = A::Err;
    type Res = B::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .map(|l| l.into())
            .or_else(|_| self.1.apply(input))
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
    fn or() {
        let r = rule('@') | '#';
        assert_eq!(apply(&r, "@"), Ruled::Ok("@", ""));
        assert_eq!(apply(&r, "#"), Ruled::Ok("#", ""));
        assert_eq!(apply(&r, "$"), Ruled::Err(()));

        let r = rule("qwe") | "123" | "null";
        assert_eq!(apply(&r, "qwe"), Ruled::Ok("qwe", ""));
        assert_eq!(apply(&r, "1234"), Ruled::Ok("123", "4"));
        assert_eq!(apply(&r, "nullable"), Ruled::Ok("null", "able"));
        assert_eq!(apply(&r, "qw"), Ruled::Err(()));
    }
}
