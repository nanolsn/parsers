use std::ops::{BitXor, RangeInclusive, RangeFrom, RangeToInclusive, RangeTo, RangeFull};
use crate::{Comply, Parser, Rule};

#[derive(Copy, Clone, Debug)]
pub struct RangeVec<R> {
    pub(crate) rule: R,
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
}

impl<R> RangeVec<R> {
    pub const fn new(rule: R, from: usize, to: Option<usize>) -> Self {
        RangeVec { rule, from, to }
    }
}

impl<'p, R> Comply<'p> for RangeVec<R>
    where
        R: Comply<'p>,
        R::On: Copy,
{
    type Res = Vec<R::Res>;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let mut count = 0;
        let mut v = Vec::new();

        loop {
            if self.to.is_some() && count >= self.to.unwrap() {
                break Ok(v);
            }

            match self.rule.comply(parser) {
                Ok(o) => {
                    count += 1;
                    v.push(o);
                }
                Err(e) => {
                    break if count >= self.from {
                        Ok(v)
                    } else {
                        parser.set_pos(pos);
                        Err(e)
                    };
                }
            }
        }
    }
}

impl<R> BitXor<std::ops::Range<usize>> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> BitXor<RangeInclusive<usize>> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, rhs: RangeInclusive<usize>) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<R> BitXor<RangeTo<usize>> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, rhs: RangeTo<usize>) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> BitXor<RangeToInclusive<usize>> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, rhs: RangeToInclusive<usize>) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<R> BitXor<RangeFrom<usize>> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, rhs: RangeFrom<usize>) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<R> BitXor<RangeFull> for Rule<R> {
    type Output = Rule<RangeVec<R>>;

    fn bitxor(self, _: RangeFull) -> Self::Output {
        Rule(RangeVec {
            rule: self.0,
            from: 0,
            to: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn range() {
        let r = rule("a") ^ (1..3);

        assert_eq!(
            Parser::new("~").parse(r),
            (Err(()), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok(vec!["a", "a"]), "a"),
        );

        let r = rule("a") ^ (0..3);

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok(vec![]), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok(vec!["a", "a"]), "a"),
        );
    }

    #[test]
    fn range_inclusive() {
        let r = rule("a") ^ (0..=0);

        assert_eq!(
            Parser::new(".").parse(r),
            (Ok(vec![]), "."),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec![]), "a"),
        );

        let r = rule("a") ^ (0..=2);

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok(vec![]), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok(vec!["a", "a"]), "a"),
        );
    }

    #[test]
    fn range_to() {
        let r = rule("a") ^ ..2;

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok(vec![]), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a"]), "a"),
        );
    }

    #[test]
    fn range_to_inclusive() {
        let r = rule("a") ^ ..=1;

        assert_eq!(
            Parser::new("~").parse(r),
            (Ok(vec![]), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a"]), "a"),
        );
    }

    #[test]
    fn range_from() {
        let r = rule("a") ^ (2..);

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
            (Ok(vec!["a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok(vec!["a", "a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaaa").parse(r),
            (Ok(vec!["a", "a", "a", "a"]), ""),
        );
    }

    #[test]
    fn range_full() {
        let r = rule("a") ^ ..;

        assert_eq!(
            Parser::new("").parse(r),
            (Ok(vec![]), ""),
        );
        assert_eq!(
            Parser::new("~").parse(r),
            (Ok(vec![]), "~"),
        );
        assert_eq!(
            Parser::new("a").parse(r),
            (Ok(vec!["a"]), ""),
        );
        assert_eq!(
            Parser::new("aa").parse(r),
            (Ok(vec!["a", "a"]), ""),
        );
        assert_eq!(
            Parser::new("aaa").parse(r),
            (Ok(vec!["a", "a", "a"]), ""),
        );
    }
}
