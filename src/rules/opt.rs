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

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        match self.0.apply(input) {
            Ruled::Ok(r, i) => Ruled::Ok(Some(r), i),
            Ruled::Err(_) => Ruled::Ok(None, input),
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
        assert_eq!(apply(&r, "qw"), Ruled::Ok(Some("q"), "w"));
        assert_eq!(apply(&r, "w"), Ruled::Ok(None, "w"));
    }
}
