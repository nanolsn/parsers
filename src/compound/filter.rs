use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Filter<R, F>(pub R, pub F);

impl<I, R, F> Rule<I> for Filter<R, F>
    where
        R: Rule<I>,
        F: FnOnce(&R::Mat) -> bool,
{
    type Exp = Option<R::Exp>;
    type Mat = R::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let Filter(p, f) = self;

        p.rule(input)
            .map_exp(|e| Some(e))
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
        rul::rul,
        SomeOf,
    };

    #[test]
    fn filter() {
        let r = (rul("@") | "#").filter(|s: &&str| *s == "@");
        assert_eq!(r.rule("@"), Ruled::Match("@", ""));
        assert_eq!(r.rule("#"), Ruled::Expected(None));
        assert_eq!(r.rule("!"), Ruled::Expected(Some(SomeOf::Str("#"))));
    }
}
