use crate::prelude::*;

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
/// # use parsers::prelude::*;
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
pub fn char_range<R>(rng: R) -> CharRange
    where
        R: std::ops::RangeBounds<char>,
{
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

    CharRange {
        from: cloned(rng.start_bound()),
        to: cloned(rng.end_bound()),
    }
}

impl<'r, 'i> Rule<'r, &'i str> for CharRange {
    type Mat = &'i str;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        use std::ops::RangeBounds;

        let CharRange { from, to } = self;

        match input.chars().next() {
            Some(c) if (*from, *to).contains(&c) => input.split_at(c.len_utf8()).into(),
            _ => Expected(Failed::CharRange(*from, *to))
        }
    }
}

impl_ops!(CharRange);

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound;

    #[test]
    fn char_range() {
        let r = super::char_range('b'..='d');
        let rng = (Bound::Included('b'), Bound::Included('d'));
        assert_eq!(r.rule("a"), Expected(Failed::CharRange(rng.0, rng.1)));
        assert_eq!(r.rule("b"), Match("b", ""));
        assert_eq!(r.rule("c"), Match("c", ""));
        assert_eq!(r.rule("d"), Match("d", ""));
        assert_eq!(r.rule("e"), Expected(Failed::CharRange(rng.0, rng.1)));

        let r = super::char_range('b'..'d');
        let rng = (Bound::Included('b'), Bound::Excluded('d'));
        assert_eq!(r.rule("a"), Expected(Failed::CharRange(rng.0, rng.1)));
        assert_eq!(r.rule("b"), Match("b", ""));
        assert_eq!(r.rule("c"), Match("c", ""));
        assert_eq!(r.rule("d"), Expected(Failed::CharRange(rng.0, rng.1)));

        let r = super::char_range(..);
        assert_eq!(r.rule("a"), Match("a", ""));
        assert_eq!(r.rule("e"), Match("e", ""));
    }
}
