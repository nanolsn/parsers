use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Pred<R, F>(pub R, pub F);

impl<I, R, F> Apply<I> for Pred<R, F>
    where
        R: Apply<I>,
        F: FnOnce(&R::Res) -> bool,
{
    type Err = Option<R::Err>;
    type Res = R::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Pred(p, f) = self;

        p.apply(input)
            .map_err(|e| Some(e))
            .and_then(|r, i| if f(&r) {
                Ruled::Match(r, i)
            } else {
                Ruled::Expected(None)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
        expected::Expected,
    };

    //noinspection RsBorrowChecker
    #[test]
    fn pred() {
        let r = (rule("@") | "#").pred(|s: &&str| *s == "@");
        assert_eq!(apply(r, "@"), Ruled::Match("@", ""));
        assert_eq!(apply(r, "#"), Ruled::Expected(None));
        assert_eq!(apply(r, "!"), Ruled::Expected(Some(Expected::Str("#"))));
    }
}
