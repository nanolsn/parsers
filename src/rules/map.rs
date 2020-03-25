use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct Map<R, F>(pub R, pub F);

impl<I, R, F, K> Apply<I> for Map<R, F>
    where
        R: Apply<I>,
        F: Fn(R::Res) -> K,
{
    type Err = R::Err;
    type Res = K;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .map(|r| (self.1)(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    //noinspection RsBorrowChecker
    #[test]
    fn map() {
        use std::str::FromStr;

        let r = (rule('1') | '2')
            .map(|s: &str| i32::from_str(s).unwrap());

        assert_eq!(apply(r, "1"), Ruled::Ok(1, ""));
        assert_eq!(apply(r, "2"), Ruled::Ok(2, ""));
        assert_eq!(apply(r, "3"), Ruled::Err(()));
    }
}
