use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
    expected::Expected,
};

#[derive(Copy, Clone, Debug)]
pub struct CharRange {
    pub(crate) from: std::ops::Bound<char>,
    pub(crate) to: std::ops::Bound<char>,
}

pub fn char_range<R>(rng: R) -> Rule<CharRange>
    where
        R: std::ops::RangeBounds<char>,
{
    Rule(CharRange {
        from: cloned(rng.start_bound()),
        to: cloned(rng.end_bound()),
    })
}

impl<'i> Apply<&'i str> for CharRange {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        use std::ops::RangeBounds;

        let rng = (self.from, self.to);

        match input.chars().next() {
            Some(c) if rng.contains(&c) => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Err(Expected::CharRange(self.from, self.to))
        }
    }
}

fn cloned<T>(bound: std::ops::Bound<&T>) -> std::ops::Bound<T>
    where
        T: Copy,
{
    use std::ops::Bound;

    match bound {
        Bound::Included(&c) => Bound::Included(c),
        Bound::Excluded(&c) => Bound::Excluded(c),
        Bound::Unbounded => Bound::Unbounded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;
    use std::ops::Bound;

    #[test]
    fn char_range() {
        let r = super::char_range('b'..='d');
        let rng = (Bound::Included('b'), Bound::Included('d'));
        assert_eq!(apply(r, "a"), Ruled::Err(Expected::CharRange(rng.0, rng.1)));
        assert_eq!(apply(r, "b"), Ruled::Ok("b", ""));
        assert_eq!(apply(r, "c"), Ruled::Ok("c", ""));
        assert_eq!(apply(r, "d"), Ruled::Ok("d", ""));
        assert_eq!(apply(r, "e"), Ruled::Err(Expected::CharRange(rng.0, rng.1)));

        let r = super::char_range('b'..'d');
        let rng = (Bound::Included('b'), Bound::Excluded('d'));
        assert_eq!(apply(r, "a"), Ruled::Err(Expected::CharRange(rng.0, rng.1)));
        assert_eq!(apply(r, "b"), Ruled::Ok("b", ""));
        assert_eq!(apply(r, "c"), Ruled::Ok("c", ""));
        assert_eq!(apply(r, "d"), Ruled::Err(Expected::CharRange(rng.0, rng.1)));

        let r = super::char_range(..);
        assert_eq!(apply(r, "a"), Ruled::Ok("a", ""));
        assert_eq!(apply(r, "e"), Ruled::Ok("e", ""));
    }
}
