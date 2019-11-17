use crate::{Comply, Parser};
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug)]
pub struct CharRange {
    pub(crate) from: char,
    pub(crate) to: char,
}

impl<'p> Comply<'p> for CharRange {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c) if self.from <= c && c <= self.to => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

pub fn char_range(r: RangeInclusive<char>) -> CharRange {
    CharRange {
        from: *r.start(),
        to: *r.end(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_range() {
        let r = super::char_range('b'..='d');

        assert_eq!(
            Parser::new("a").parse(r),
            (Err(()), "a"),
        );
        assert_eq!(
            Parser::new("b").parse(r),
            (Ok("b"), ""),
        );
        assert_eq!(
            Parser::new("c").parse(r),
            (Ok("c"), ""),
        );
        assert_eq!(
            Parser::new("d").parse(r),
            (Ok("d"), ""),
        );
        assert_eq!(
            Parser::new("e").parse(r),
            (Err(()), "e"),
        );
    }
}
