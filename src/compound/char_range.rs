use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

/// Char range parser.
///
/// Checks that the input `char` is in the specified range.
/// Use [`char_range`] to create a range.
///
/// [`char_range`]: ./fn.char_range.html
///
/// # Examples
///
/// ```
/// # use parsers::{compound::char_range, Rule};
/// // Match any letter from `a` to `f`
/// let rule = char_range('a'..='f');
///
/// assert!(rule.rule("a").is_match());
/// assert!(rule.rule("f").is_match());
/// assert!(rule.rule("g").is_expected());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct CharRange {
    pub(crate) from: std::ops::Bound<char>,
    pub(crate) to: std::ops::Bound<char>,
}

/// [`CharRange`] constructor. See [`CharRange`] for details.
///
/// [`CharRange`]: ./struct.CharRange.html
pub fn char_range<R>(rng: R) -> Rul<CharRange>
    where
        R: std::ops::RangeBounds<char>,
{
    Rul(CharRange {
        from: cloned(rng.start_bound()),
        to: cloned(rng.end_bound()),
    })
}

impl<'i> Rule<&'i str> for CharRange {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        use std::ops::RangeBounds;

        let rng = (self.from, self.to);

        match input.chars().next() {
            Some(c) if rng.contains(&c) => input.split_at(c.len_utf8()).into(),
            _ => Ruled::Expected(SomeOf::CharRange(self.from, self.to))
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
    use std::ops::Bound;

    #[test]
    fn char_range() {
        let r = super::char_range('b'..='d');
        let rng = (Bound::Included('b'), Bound::Included('d'));
        assert_eq!(r.rule("a"), Ruled::Expected(SomeOf::CharRange(rng.0, rng.1)));
        assert_eq!(r.rule("b"), Ruled::Match("b", ""));
        assert_eq!(r.rule("c"), Ruled::Match("c", ""));
        assert_eq!(r.rule("d"), Ruled::Match("d", ""));
        assert_eq!(r.rule("e"), Ruled::Expected(SomeOf::CharRange(rng.0, rng.1)));

        let r = super::char_range('b'..'d');
        let rng = (Bound::Included('b'), Bound::Excluded('d'));
        assert_eq!(r.rule("a"), Ruled::Expected(SomeOf::CharRange(rng.0, rng.1)));
        assert_eq!(r.rule("b"), Ruled::Match("b", ""));
        assert_eq!(r.rule("c"), Ruled::Match("c", ""));
        assert_eq!(r.rule("d"), Ruled::Expected(SomeOf::CharRange(rng.0, rng.1)));

        let r = super::char_range(..);
        assert_eq!(r.rule("a"), Ruled::Match("a", ""));
        assert_eq!(r.rule("e"), Ruled::Match("e", ""));
    }
}
