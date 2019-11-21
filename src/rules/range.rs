use std::ops::{Mul, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull};
use crate::{Comply, Parser, Rule};

#[derive(Copy, Clone, Debug)]
pub struct Range<R> {
    pub(crate) rule: R,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<R> Range<R> {
    pub const fn new(rule: R, from: usize, to: Option<usize>) -> Self {
        Range { rule, from, to }
    }
}

impl<'p, R, S> Comply<'p> for Range<R>
    where
        R: Comply<'p, Res=S>,
        S: AsRef<str> + 'p,
        R::On: Copy,
{
    type Res = String;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let mut count = 0;
        let mut s = String::new();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok(s);
            }

            match self.rule.comply(parser) {
                Ok(o) => {
                    count += 1;
                    s.push_str(o.as_ref());
                }
                Err(e) => {
                    break if count >= self.from {
                        Ok(s)
                    } else {
                        parser.set_pos(pos);
                        Err(e)
                    };
                }
            }
        }
    }
}

impl<R> Mul<std::ops::Range<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> Mul<RangeInclusive<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: RangeInclusive<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<R> Mul<RangeTo<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: RangeTo<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> Mul<RangeToInclusive<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: RangeToInclusive<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<R> Mul<RangeFrom<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: RangeFrom<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<R> Mul<RangeFull> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, _: RangeFull) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_res;

    #[test]
    fn range() {
        let r = string_res("a") * (1..3);

        assert_eq!(
            Parser::new("~").parse(r),
            (Err(()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("aa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok("aa".to_string()), "a"),
        );

        let r = string_res("a") * (0..3);

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok("".to_string()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("aa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok("aa".to_string()), "a"),
        );
    }

    #[test]
    fn range_inclusive() {
        let r = string_res("a") * (0..=0);

        assert_eq!(
            Parser::new(".").parse(r),
            (Ok("".to_string()), "."),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("".to_string()), "a"),
        );

        let r = string_res("a") * (0..=2);

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok("".to_string()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("aa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok("aa".to_string()), "a"),
        );
    }

    #[test]
    fn range_to() {
        let r = string_res("a") * ..2;

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok("".to_string()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("a".to_string()), "a"),
        );
    }

    #[test]
    fn range_to_inclusive() {
        let r = string_res("a") * ..=1;

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok("".to_string()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("a".to_string()), "a"),
        );
    }

    #[test]
    fn range_from() {
        let r = string_res("a") * (2..);

        assert_eq!(
            Parser::new("").parse(r),
            (Err(()), ""),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Err(()), "a"),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("aa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok("aaa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaaa").parse(r),
            (Ok("aaaa".to_string()), ""),
        );
    }

    #[test]
    fn range_full() {
        let r = string_res("a") * ..;

        assert_eq!(
            Parser::new("").parse(r),
            (Ok("".to_string()), ""),
        );
        assert_eq!(
            Parser::new("~").parse(r),
            (Ok("".to_string()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok("a".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok("aa".to_string()), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok("aaa".to_string()), ""),
        );
    }
}
