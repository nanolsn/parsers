use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Filter<R, F>(pub R, pub F);

impl<'r, I: 'r, R, F> Rule<'r, I> for Filter<R, F>
    where
        R: Rule<'r, I>,
        R::Exp: Into<Failed<'r>>,
        F: Fn(&R::Mat) -> bool,
{
    type Mat = R::Mat;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .map_exp(|e| e.into())
            .and_then(|r, i| if (self.1)(&r) {
                Match(r, i)
            } else {
                Expected(Failed::Predicate)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter() {
        let r = "@".or("#").filter(|&s| s == "@");
        assert_eq!(r.rule("@"), Match("@", ""));
        assert_eq!(r.rule("#"), Expected(Failed::Predicate));
        assert_eq!(r.rule("!"), Expected(Failed::Str("#")));
    }
}
