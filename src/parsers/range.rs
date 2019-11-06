use crate::{Parse, Parser};
use std::ops::{Mul, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull};

#[derive(Copy, Clone, Debug)]
pub struct Range<P> {
    pub(crate) parser: P,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<P, I> Parse<I> for Range<P>
    where
        P: Parse<I>,
        I: Copy,
{
    type Err = P::Err;
    type Out = Vec<P::Out>;

    fn parse(&self, mut rest: I) -> Result<(Self::Out, I), Self::Err> {
        let mut count = 0;
        let mut v = Vec::with_capacity(if let Some(to) = self.to {
            to
        } else {
            self.from
        });

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok((v, rest))
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
                    }
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
    use crate::par;

    #[test]
    fn range() {
        let r = par("!") * (1..3);

        assert_eq!(r.parse("~"), Err(()));
        assert_eq!(r.parse("!"), Ok((vec!["!"], "")));
        assert_eq!(r.parse("!!"), Ok((vec!["!", "!"], "")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!", "!"], "!")));

        let r = par("!") * (0..3);

        assert_eq!(r.parse("~"), Ok((vec![], "~")));
        assert_eq!(r.parse("!"), Ok((vec!["!"], "")));
        assert_eq!(r.parse("!!"), Ok((vec!["!", "!"], "")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!", "!"], "!")));
    }

    #[test]
    fn range_inclusive() {
        let r = par("!") * (0..=0);

        assert_eq!(r.parse("."), Ok((vec![], ".")));
        assert_eq!(r.parse("!"), Ok((vec![], "!")));

        let r = par("!") * (0..=2);

        assert_eq!(r.parse("~"), Ok((vec![], "~")));
        assert_eq!(r.parse("!"), Ok((vec!["!"], "")));
        assert_eq!(r.parse("!!"), Ok((vec!["!", "!"], "")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!", "!"], "!")));
    }

    #[test]
    fn range_to() {
        let r = par("!") * ..2;

        assert_eq!(r.parse("~"), Ok((vec![], "~")));
        assert_eq!(r.parse("!!"), Ok((vec!["!"], "!")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!"], "!!")));
    }

    #[test]
    fn range_to_inclusive() {
        let r = par("!") * ..=1;

        assert_eq!(r.parse("~"), Ok((vec![], "~")));
        assert_eq!(r.parse("!!"), Ok((vec!["!"], "!")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!"], "!!")));
    }

    #[test]
    fn range_from() {
        let r = par("!") * (2..);

        assert_eq!(r.parse(""), Err(()));
        assert_eq!(r.parse("!"), Err(()));
        assert_eq!(r.parse("!!"), Ok((vec!["!", "!"], "")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!", "!", "!"], "")));
        assert_eq!(r.parse("!!!!"), Ok((vec!["!", "!", "!", "!"], "")));
    }

    #[test]
    fn range_full() {
        let r = par("!") * ..;

        assert_eq!(r.parse(""), Ok((vec![], "")));
        assert_eq!(r.parse("~"), Ok((vec![], "~")));
        assert_eq!(r.parse("!~"), Ok((vec!["!"], "~")));
        assert_eq!(r.parse("!"), Ok((vec!["!"], "")));
        assert_eq!(r.parse("!!"), Ok((vec!["!", "!"], "")));
        assert_eq!(r.parse("!!!"), Ok((vec!["!", "!", "!"], "")));
        assert_eq!(r.parse("!!!!"), Ok((vec!["!", "!", "!", "!"], "")));
    }
}
