use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct MapErr<R, F>(pub R, pub F);

impl<I, R, F, Q> Apply<I> for MapErr<R, F>
    where
        R: Apply<I>,
        F: Fn(R::Err) -> Q,
{
    type Err = Q;
    type Res = R::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .map_err(|e| (self.1)(e))
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
    fn map_err() {
        let r = rule('1').map_err(|_| 1);
        assert_eq!(apply(r, "1"), Ruled::Ok("1", ""));
        assert_eq!(apply(r, "2"), Ruled::Err(1));
    }
}
