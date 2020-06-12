use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct MapExp<R, F>(pub R, pub F);

impl<'r, I: 'r, R, F, Q> Rule<'r, I> for MapExp<R, F>
    where
        R: Rule<'r, I>,
        F: Fn(R::Exp) -> Q,
{
    type Mat = R::Mat;
    type Exp = Q;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .map_exp(|e| (self.1)(e))
    }
}

impl_ops!(MapExp<R, F>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_exp() {
        let r = '1'.map_exp(|_| 1);
        assert_eq!(r.rule("1"), Match("1", ""));
        assert_eq!(r.rule("2"), Expected(1));
    }
}
