use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Opt<R>(pub R);

impl<I, R> Apply<I> for Opt<R>
    where
        R: Apply<I>,
        I: Copy,
{
    type Err = R::Err;
    type Res = Option<R::Res>;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.apply(input) {
            Ruled::Match(r, i) => Ruled::Match(Some(r), i),
            Ruled::Expected(_) => Ruled::Match(None, input),
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
    fn opt() {
        let r = rule("q").opt();
        assert_eq!(apply(r, "qw"), Ruled::Match(Some("q"), "w"));
        assert_eq!(apply(r, "w"), Ruled::Match(None, "w"));
    }
}
