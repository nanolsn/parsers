use crate::{Parse, Parser, Parsed};
use std::ops::{Mul, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull};

#[derive(Copy, Clone, Debug)]
pub struct Range<P> {
    pub(crate) parser: P,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<'p, P, S> Parse<'p> for Range<P>
    where
        P: Parse<'p, Res=S>,
        S: AsRef<str> + 'p,
        P::On: Copy,
{
    type Res = String;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, mut rest: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        let mut count = 0;
        let mut s = String::new();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok((s, rest));
            }

            match self.parser.parse(rest) {
                Ok((out, r)) => {
                    count += 1;
                    rest = r;
                    s.push_str(out.as_ref());
                }
                Err(e) => {
                    break if count >= self.from {
                        Ok((s, rest))
                    } else {
                        Err(e)
                    };
                }
            }
        }
    }
}

impl<P> Mul<std::ops::Range<usize>> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> Mul<RangeInclusive<usize>> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, rhs: RangeInclusive<usize>) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<P> Mul<RangeTo<usize>> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, rhs: RangeTo<usize>) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> Mul<RangeToInclusive<usize>> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, rhs: RangeToInclusive<usize>) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<P> Mul<RangeFrom<usize>> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, rhs: RangeFrom<usize>) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<P> Mul<RangeFull> for Parser<P> {
    type Output = Parser<Range<P>>;

    fn mul(self, _: RangeFull) -> Self::Output {
        Parser(Range {
            parser: self.0,
            from: 0,
            to: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stringed_par;

    #[test]
    fn range() {
        let r = stringed_par("a") * (1..3);

        assert_eq!(r.parse("~"), Err(()));
        assert_eq!(r.parse("a"), Ok(("a".to_string(), "")));
        assert_eq!(r.parse("aa"), Ok(("aa".to_string(), "")));
        assert_eq!(r.parse("aaa"), Ok(("aa".to_string(), "a")));

        let r = stringed_par("a") * (0..3);

        assert_eq!(r.parse("~"), Ok(("".to_string(), "~")));
        assert_eq!(r.parse("a"), Ok(("a".to_string(), "")));
        assert_eq!(r.parse("aa"), Ok(("aa".to_string(), "")));
        assert_eq!(r.parse("aaa"), Ok(("aa".to_string(), "a")));
    }

    #[test]
    fn range_inclusive() {
        let r = stringed_par("a") * (0..=0);

        assert_eq!(r.parse("."), Ok(("".to_string(), ".")));
        assert_eq!(r.parse("a"), Ok(("".to_string(), "a")));

        let r = stringed_par("a") * (0..=2);

        assert_eq!(r.parse("~"), Ok(("".to_string(), "~")));
        assert_eq!(r.parse("a"), Ok(("a".to_string(), "")));
        assert_eq!(r.parse("aa"), Ok(("aa".to_string(), "")));
        assert_eq!(r.parse("aaa"), Ok(("aa".to_string(), "a")));
    }

    #[test]
    fn range_to() {
        let r = stringed_par("a") * ..2;

        assert_eq!(r.parse("~"), Ok(("".to_string(), "~")));
        assert_eq!(r.parse("aa"), Ok(("a".to_string(), "a")));
        assert_eq!(r.parse("aaa"), Ok(("a".to_string(), "aa")));
    }

    #[test]
    fn range_to_inclusive() {
        let r = stringed_par("a") * ..=1;

        assert_eq!(r.parse("~"), Ok(("".to_string(), "~")));
        assert_eq!(r.parse("aa"), Ok(("a".to_string(), "a")));
        assert_eq!(r.parse("aaa"), Ok(("a".to_string(), "aa")));
    }

    #[test]
    fn range_from() {
        let r = stringed_par("a") * (2..);

        assert_eq!(r.parse(""), Err(()));
        assert_eq!(r.parse("a"), Err(()));
        assert_eq!(r.parse("aa"), Ok(("aa".to_string(), "")));
        assert_eq!(r.parse("aaa"), Ok(("aaa".to_string(), "")));
        assert_eq!(r.parse("aaaa"), Ok(("aaaa".to_string(), "")));
    }

    #[test]
    fn range_full() {
        let r = stringed_par("a") * ..;

        assert_eq!(r.parse(""), Ok(("".to_string(), "")));
        assert_eq!(r.parse("~"), Ok(("".to_string(), "~")));
        assert_eq!(r.parse("a~"), Ok(("a".to_string(), "~")));
        assert_eq!(r.parse("a"), Ok(("a".to_string(), "")));
        assert_eq!(r.parse("aa"), Ok(("aa".to_string(), "")));
        assert_eq!(r.parse("aaa"), Ok(("aaa".to_string(), "")));
        assert_eq!(r.parse("aaaa"), Ok(("aaaa".to_string(), "")));
    }
}
