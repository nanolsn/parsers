use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Map<R, F>(pub R, pub F);

impl<'r, I: 'r, R, F, K> Rule<'r, I> for Map<R, F>
    where
        R: Rule<'r, I>,
        F: Fn(R::Mat) -> K,
{
    type Mat = K;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .map(|r| (self.1)(r))
    }
}

impl_ops!(Map<R, F>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {
        use std::str::FromStr;

        let r = '1'.or('2').map(|s| i32::from_str(s).unwrap());
        assert_eq!(r.rule("1"), Match(1, ""));
        assert_eq!(r.rule("2"), Match(2, ""));
        assert_eq!(r.rule("3"), Expected(Failed::Char('2')));
    }
}
