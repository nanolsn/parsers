use crate::{
    rule::Rule,
    ruled::Ruled,
    concat::Concat,
};

#[derive(Debug)]
pub struct Range<T, R> {
    pub(crate) rule: R,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
    pub(crate) phantom: std::marker::PhantomData<*const T>,
}

impl<T, R> Range<T, R> {
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

impl<T, R> Clone for Range<T, R>
    where
        R: Clone,
{
    fn clone(&self) -> Self { Range::new(self.rule.clone(), self.from, self.to) }
}

impl<T, R> Copy for Range<T, R>
    where
        R: Copy,
{}

impl<I, T, R> Rule<I> for Range<T, R>
    where
        R: Rule<I> + Copy,
        I: Copy,
        T: Concat<T, R::Mat>,
{
    type Exp = R::Exp;
    type Mat = T;

    fn rule(self, mut input: I) -> Ruled<I, Self::Res, Self::Err> {
        let mut count = 0;
        let mut res = T::empty();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ruled::Match(res, input);
            }

            match self.rule.rule(input) {
                Ruled::Match(r, i) => {
                    count += 1;
                    input = i;
                    res = T::concat(res, r);
                }
                Ruled::Expected(e) => {
                    break if count >= self.from {
                        Ruled::Match(res, input)
                    } else {
                        Ruled::Expected(e)
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
        rul::rul,
        SomeOf,
    };

    #[test]
    fn range() {
        let r = rul("a") * (1..3);
        assert_eq!(r.rule("~"), Ruled::Expected(SomeOf::Str("a")));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Ruled::Match("aa".to_owned(), "a"));

        let r = rul("a") * (0..3);
        assert_eq!(r.rule("~"), Ruled::Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Ruled::Match("aa".to_owned(), "a"));
    }

    #[test]
    fn range_inclusive() {
        let r = rul("a") * (0..=0);
        assert_eq!(r.rule("."), Ruled::Match("".to_owned(), "."));
        assert_eq!(r.rule("a"), Ruled::Match("".to_owned(), "a"));

        let r = rul("a") * (0..=2);
        assert_eq!(r.rule("~"), Ruled::Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Ruled::Match("aa".to_owned(), "a"));
    }

    #[test]
    fn range_to() {
        let r = rul("a") * ..2;
        assert_eq!(r.rule("~"), Ruled::Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("a".to_owned(), "a"));
    }

    #[test]
    fn range_to_inclusive() {
        let r = rul("a") * ..=1;
        assert_eq!(r.rule("~"), Ruled::Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("a".to_owned(), "a"));
    }

    #[test]
    fn range_from() {
        let r = rul("a") * (2..);
        assert_eq!(r.rule(""), Ruled::Expected(SomeOf::Str("a")));
        assert_eq!(r.rule("a"), Ruled::Expected(SomeOf::Str("a")));
        assert_eq!(r.rule("aa"), Ruled::Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Ruled::Match("aaa".to_owned(), ""));
    }

    #[test]
    fn range_full() {
        let r = rul("a") * ..;
        assert_eq!(r.rule(""), Ruled::Match("".to_owned(), ""));
        assert_eq!(r.rule("~"), Ruled::Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Ruled::Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Ruled::Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Ruled::Match("aaa".to_owned(), ""));
    }
}
