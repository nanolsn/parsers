use crate::{
    prelude::*,
    Concat,
};

#[derive(Debug)]
pub struct Range<R, C> {
    rule: R,
    from: usize,
    to: Option<usize>,
    phantom: std::marker::PhantomData<C>,
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

impl<'r, I: 'r, R, C> Rule<'r, I> for Range<R, C>
    where
        R: Rule<'r, I>,
        I: Copy,
        C: Concat<C, R::Mat>,
{
    type Mat = C;
    type Exp = R::Exp;

    fn rule(&'r self, mut input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        let mut count = 0;
        let mut res = C::empty();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Match(res, input);
            }

            match self.rule.rule(input) {
                Match(r, i) => {
                    count += 1;
                    input = i;
                    res = C::concat(res, r);
                }
                Expected(e) => {
                    break if count >= self.from {
                        Match(res, input)
                    } else {
                        Expected(e)
                    };
                }
            }
        }
    }
}

impl_or!(Range<R, C>);
impl_shifts!(Range<R, C>);
impl_not!(Range<R, C>);

impl<R, T> std::ops::BitAnd<T> for Range<R, &'static str> {
    type Output = super::Cat<Range<R, &'static str>, T, &'static str>;

    fn bitand(self, rhs: T) -> Self::Output { super::Cat::new(self, rhs) }
}

impl<R, T> std::ops::Add<T> for Range<R, String> {
    type Output = super::Cat<Range<R, String>, T, String>;

    fn add(self, rhs: T) -> Self::Output { super::Cat::new(self, rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let r = "a".range(1..3);
        assert_eq!(r.rule("~"), Expected(Failed::Str("a")));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Match("aa".to_owned(), "a"));

        let r = "a".range(0..3);
        assert_eq!(r.rule("~"), Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Match("aa".to_owned(), "a"));
    }

    #[test]
    fn range_inclusive() {
        let r = "a".range(0..=0);
        assert_eq!(r.rule("."), Match("".to_owned(), "."));
        assert_eq!(r.rule("a"), Match("".to_owned(), "a"));

        let r = "a".range(0..=2);
        assert_eq!(r.rule("~"), Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Match("aa".to_owned(), "a"));
    }

    #[test]
    fn range_to() {
        let r = "a".range(..2);
        assert_eq!(r.rule("~"), Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("a".to_owned(), "a"));
    }

    #[test]
    fn range_to_inclusive() {
        let r = "a".range(..=1);
        assert_eq!(r.rule("~"), Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("a".to_owned(), "a"));
    }

    #[test]
    fn range_from() {
        let r = "a".range(2..);
        assert_eq!(r.rule(""), Expected(Failed::Str("a")));
        assert_eq!(r.rule("a"), Expected(Failed::Str("a")));
        assert_eq!(r.rule("aa"), Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Match("aaa".to_owned(), ""));
    }

    #[test]
    fn range_full() {
        let r = "a".range(..);
        assert_eq!(r.rule(""), Match("".to_owned(), ""));
        assert_eq!(r.rule("~"), Match("".to_owned(), "~"));
        assert_eq!(r.rule("a"), Match("a".to_owned(), ""));
        assert_eq!(r.rule("aa"), Match("aa".to_owned(), ""));
        assert_eq!(r.rule("aaa"), Match("aaa".to_owned(), ""));
    }
}
