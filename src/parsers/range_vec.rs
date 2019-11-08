use crate::{Parse, Parser};
use std::ops::{BitXor, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull, Range};

#[derive(Copy, Clone, Debug)]
pub struct RangeVec<P> {
    pub(crate) parser: P,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<P> RangeVec<P> {
    pub fn reduce<F>(self, f: F) -> Parser<Reduce<Self, F>> {
        Parser(Reduce(self, f))
    }
}

impl<P, I> Parse<I> for RangeVec<P>
    where
        P: Parse<I>,
        I: Copy,
{
    type Err = P::Err;
    type Out = Vec<P::Out>;

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
        let mut count = 0;
        let mut v = Vec::new();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok((v, rest));
            }

            match self.parser.parse(rest) {
                Ok((out, r)) => {
                    count += 1;
                    rest = r;
                    v.push(out);
                }
                Err(e) => {
                    break if count >= self.from {
                        Ok((v, rest))
                    } else {
                        Err(e)
                    };
                }
            }
        }
    }
}

impl<P> BitXor<Range<usize>> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> BitXor<RangeInclusive<usize>> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, rhs: RangeInclusive<usize>) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<P> BitXor<RangeTo<usize>> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, rhs: RangeTo<usize>) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> BitXor<RangeToInclusive<usize>> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, rhs: RangeToInclusive<usize>) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<P> BitXor<RangeFrom<usize>> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, rhs: RangeFrom<usize>) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<P> BitXor<RangeFull> for Parser<P> {
    type Output = Parser<RangeVec<P>>;

    fn bitxor(self, _: RangeFull) -> Self::Output {
        Parser(RangeVec {
            parser: self.0,
            from: 0,
            to: None,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Reduce<P, F>(pub(crate) P, pub(crate) F);

impl<P, F, I, R> Parse<I> for Reduce<P, F>
    where
        P: Parse<I, Out=Vec<R>>,
        F: Fn(R, R) -> R + Copy,
{
    type Err = P::Err;
    type Out = R;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        let (v, rest) = self.0.parse(input)?;
        assert!(v.len() > 0);

        let mut it = v.into_iter();
        let first = it.next().unwrap();
        let result = it.fold(first, self.1);

        Ok((result, rest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn range() {
        let r = par("a") ^ (1..3);

        assert!(r.parse("~").is_err());
        assert_eq!(r.parse("a").unwrap(), (vec!["a"], ""));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a", "a"], ""));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a", "a"], "a"));

        let r = par("a") ^ (0..3);

        assert_eq!(r.parse("~").unwrap(), (vec![], "~"));
        assert_eq!(r.parse("a").unwrap(), (vec!["a"], ""));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a", "a"], ""));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a", "a"], "a"));
    }

    #[test]
    fn range_inclusive() {
        let r = par("a") ^ (0..=0);

        assert_eq!(r.parse(".").unwrap(), (vec![], "."));
        assert_eq!(r.parse("a").unwrap(), (vec![], "a"));

        let r = par("a") ^ (0..=2);

        assert_eq!(r.parse("~").unwrap(), (vec![], "~"));
        assert_eq!(r.parse("a").unwrap(), (vec!["a"], ""));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a", "a"], ""));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a", "a"], "a"));
    }

    #[test]
    fn range_to() {
        let r = par("a") ^ ..2;

        assert_eq!(r.parse("~").unwrap(), (vec![], "~"));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a"], "a"));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a"], "aa"));
    }

    #[test]
    fn range_to_inclusive() {
        let r = par("a") ^ ..=1;

        assert_eq!(r.parse("~").unwrap(), (vec![], "~"));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a"], "a"));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a"], "aa"));
    }

    #[test]
    fn range_from() {
        let r = par("a") ^ (2..);

        assert!(r.parse("").is_err());
        assert!(r.parse("a").is_err());
        assert_eq!(r.parse("aa").unwrap(), (vec!["a", "a"], ""));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a", "a", "a"], ""));
        assert_eq!(r.parse("aaaa").unwrap(), (vec!["a", "a", "a", "a"], ""));
    }

    #[test]
    fn range_full() {
        let r = par("a") ^ ..;

        assert_eq!(r.parse("").unwrap(), (vec![], ""));
        assert_eq!(r.parse("~").unwrap(), (vec![], "~"));
        assert_eq!(r.parse("a~").unwrap(), (vec!["a"], "~"));
        assert_eq!(r.parse("a").unwrap(), (vec!["a"], ""));
        assert_eq!(r.parse("aa").unwrap(), (vec!["a", "a"], ""));
        assert_eq!(r.parse("aaa").unwrap(), (vec!["a", "a", "a"], ""));
        assert_eq!(r.parse("aaaa").unwrap(), (vec!["a", "a", "a", "a"], ""));
    }

    #[test]
    fn reduce() {
        let r = par("a").map(|_| 1) ^ ..;
        let r = r.reduce(|a, b| a + b);

        assert_eq!(r.parse("a").unwrap(), (1, ""));
        assert_eq!(r.parse("aa").unwrap(), (2, ""));
        assert_eq!(r.parse("aaa").unwrap(), (3, ""));
    }
}
