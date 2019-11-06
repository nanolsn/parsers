use crate::{Parse, Parser};
use std::ops::{BitXor, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull};

#[derive(Copy, Clone, Debug)]
pub struct RangeConcat<P> {
    pub(crate) parser: P,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<'i, P> Parse<&'i str> for RangeConcat<P>
    where
        P: Parse<&'i str, Out=&'i str>,
{
    type Err = P::Err;
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        let mut count = 0;
        let mut len = 0;
        let mut rest = input;

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok((&input[..len], rest))
            }

            match self.parser.parse(rest) {
                Ok((out, r)) => {
                    count += 1;
                    rest = r;
                    len += out.len();
                }
                Err(e) => {
                    break if count >= self.from {
                        Ok((&input[..len], rest))
                    } else {
                        Err(e)
                    }
                }
            }
        }
    }
}

impl<P> BitXor<std::ops::Range<usize>> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Parser(RangeConcat {
            parser: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> BitXor<RangeInclusive<usize>> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, rhs: RangeInclusive<usize>) -> Self::Output {
        Parser(RangeConcat {
            parser: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<P> BitXor<RangeTo<usize>> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, rhs: RangeTo<usize>) -> Self::Output {
        Parser(RangeConcat {
            parser: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<P> BitXor<RangeToInclusive<usize>> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, rhs: RangeToInclusive<usize>) -> Self::Output {
        Parser(RangeConcat {
            parser: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<P> BitXor<RangeFrom<usize>> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, rhs: RangeFrom<usize>) -> Self::Output {
        Parser(RangeConcat {
            parser: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<P> BitXor<RangeFull> for Parser<P> {
    type Output = Parser<RangeConcat<P>>;

    fn bitxor(self, _: RangeFull) -> Self::Output {
        Parser(RangeConcat {
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
    fn range_concat() {
        let r = par("!") ^ (1..3);

        assert_eq!(r.parse("~"), Err(()));
        assert_eq!(r.parse("!"), Ok(("!", "")));
        assert_eq!(r.parse("!!"), Ok(("!!", "")));
        assert_eq!(r.parse("!!!"), Ok(("!!", "!")));

        let r = par("!") ^ (0..3);

        assert_eq!(r.parse("~"), Ok(("", "~")));
        assert_eq!(r.parse("!"), Ok(("!", "")));
        assert_eq!(r.parse("!!"), Ok(("!!", "")));
        assert_eq!(r.parse("!!!"), Ok(("!!", "!")));
    }

    #[test]
    fn range_inclusive_concat() {
        let r = par("!") ^ (0..=0);

        assert_eq!(r.parse("."), Ok(("", ".")));
        assert_eq!(r.parse("!"), Ok(("", "!")));

        let r = par("!") ^ (0..=2);

        assert_eq!(r.parse("~"), Ok(("", "~")));
        assert_eq!(r.parse("!"), Ok(("!", "")));
        assert_eq!(r.parse("!!"), Ok(("!!", "")));
        assert_eq!(r.parse("!!!"), Ok(("!!", "!")));
    }

    #[test]
    fn range_to_concat() {
        let r = par("!") ^ ..2;

        assert_eq!(r.parse("~"), Ok(("", "~")));
        assert_eq!(r.parse("!!"), Ok(("!", "!")));
        assert_eq!(r.parse("!!!"), Ok(("!", "!!")));
    }

    #[test]
    fn range_to_inclusive_concat() {
        let r = par("!") ^ ..=1;

        assert_eq!(r.parse("~"), Ok(("", "~")));
        assert_eq!(r.parse("!!"), Ok(("!", "!")));
        assert_eq!(r.parse("!!!"), Ok(("!", "!!")));
    }

    #[test]
    fn range_from_concat() {
        let r = par("!") ^ (2..);

        assert_eq!(r.parse(""), Err(()));
        assert_eq!(r.parse("!"), Err(()));
        assert_eq!(r.parse("!!"), Ok(("!!", "")));
        assert_eq!(r.parse("!!!"), Ok(("!!!", "")));
        assert_eq!(r.parse("!!!!"), Ok(("!!!!", "")));
    }

    #[test]
    fn range_full_concat() {
        let r = par("!") ^ ..;

        assert_eq!(r.parse(""), Ok(("", "")));
        assert_eq!(r.parse("~"), Ok(("", "~")));
        assert_eq!(r.parse("!~"), Ok(("!", "~")));
        assert_eq!(r.parse("!"), Ok(("!", "")));
        assert_eq!(r.parse("!!"), Ok(("!!", "")));
        assert_eq!(r.parse("!!!"), Ok(("!!!", "")));
        assert_eq!(r.parse("!!!!"), Ok(("!!!!", "")));
    }
}
