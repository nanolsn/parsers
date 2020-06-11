use crate::{
    rule::Rule,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct MapErr<R, F>(pub R, pub F);

impl<I, R, F, Q> Rule<I> for MapErr<R, F>
    where
        R: Rule<I>,
        F: FnOnce(R::Exp) -> Q,
{
    type Exp = Q;
    type Mat = R::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let MapErr(p, f) = self;

        p.rule(input)
            .map_exp(|e| f(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rul::rul;

    #[test]
    fn map_err() {
        let r = rul('1').map_err(|_| 1);
        assert_eq!(r.rule("1"), Ruled::Match("1", ""));
        assert_eq!(r.rule("2"), Ruled::Expected(1));
    }
}
