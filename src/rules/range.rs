use crate::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Debug)]
pub struct Range<R, C> {
    pub(crate) rule: R,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
    pub(crate) phantom: std::marker::PhantomData<*const C>,
}

impl<R, C> Range<R, C> {
    pub fn new(rule: R, from: usize, to: Option<usize>) -> Self {
        Range {
            rule,
            from,
            to,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn from_range<B>(rule: R, rng: B) -> Self
        where
            B: std::ops::RangeBounds<usize>,
    {
        use std::ops::Bound::*;

        let from = match rng.start_bound() {
            Included(&b) => b,
            _ => 0,
        };

        let to = match rng.end_bound() {
            Included(&b) => Some(b),
            Excluded(&b) => Some(b.saturating_sub(1)),
            Unbounded => None,
        };

        Range::new(rule, from, to)
    }
}

impl<R, C> Clone for Range<R, C>
    where
        R: Clone,
{
    fn clone(&self) -> Self { Range::new(self.rule.clone(), self.from, self.to) }
}

impl<R, C> Copy for Range<R, C>
    where
        R: Copy,
{}

impl<R, I, C> Apply<I> for Range<R, C>
    where
        R: Apply<I>,
        I: Copy,
        R::Res: AsRef<str>,
        C: Concat<C, R::Res>,
{
    type Err = R::Err;
    type Res = C;

    fn apply(&self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut count = 0;
        let mut res = Concat::empty();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ruled::Ok(res, input);
            }

            match self.rule.apply(input) {
                Ruled::Ok(r, i) => {
                    count += 1;
                    input = i;
                    res = C::concat(res, r);
                }
                Ruled::Err(e) => {
                    break if count >= self.from {
                        Ruled::Ok(res, input)
                    } else {
                        Ruled::Err(e)
                    };
                }
            }
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
    fn range() {
        let r = rule("a") * (1..3);
        assert_eq!(apply(r, "~"), Ruled::Err(()));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));

        let r = rule("a") * (0..3);
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));
    }

    #[test]
    fn range_inclusive() {
        let r = rule("a") * (0..=0);
        assert_eq!(apply(r, "."), Ruled::Ok("".to_owned(), "."));
        assert_eq!(apply(r, "a"), Ruled::Ok("".to_owned(), "a"));

        let r = rule("a") * (0..=2);
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aa".to_owned(), "a"));
    }

    #[test]
    fn range_to() {
        let r = rule("a") * ..2;
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("a".to_owned(), "a"));
    }

    #[test]
    fn range_to_inclusive() {
        let r = rule("a") * ..=1;
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("a".to_owned(), "a"));
    }

    #[test]
    fn range_from() {
        let r = rule("a") * (2..);
        assert_eq!(apply(r, ""), Ruled::Err(()));
        assert_eq!(apply(r, "a"), Ruled::Err(()));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aaa".to_owned(), ""));
    }

    #[test]
    fn range_full() {
        let r = rule("a") * ..;
        assert_eq!(apply(r, ""), Ruled::Ok("".to_owned(), ""));
        assert_eq!(apply(r, "~"), Ruled::Ok("".to_owned(), "~"));
        assert_eq!(apply(r, "a"), Ruled::Ok("a".to_owned(), ""));
        assert_eq!(apply(r, "aa"), Ruled::Ok("aa".to_owned(), ""));
        assert_eq!(apply(r, "aaa"), Ruled::Ok("aaa".to_owned(), ""));
    }
}
