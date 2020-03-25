use crate::{
    apply::Apply,
    ruled::Ruled,
};

#[derive(Copy, Clone, Debug)]
pub struct CharRange {
    pub(crate) from: std::ops::Bound<char>,
    pub(crate) to: std::ops::Bound<char>,
}

impl<'i> Apply<&'i str> for CharRange {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        use std::ops::RangeBounds;

        match input.chars().next() {
            Some(c) if (self.from, self.to).contains(&c) => {
                let (l, r) = input.split_at(c.len_utf8());
                Ruled::Ok(l, r)
            }
            _ => Ruled::Err(())
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

#[allow(dead_code)]
pub fn char_range<R>(rng: R) -> CharRange
    where
        R: std::ops::RangeBounds<char>,
{
    CharRange {
        from: cloned(rng.start_bound()),
        to: cloned(rng.end_bound()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn char_range() {
        let r = super::char_range('b'..='d');
        assert_eq!(apply(r, "a"), Ruled::Err(()));
        assert_eq!(apply(r, "b"), Ruled::Ok("b", ""));
        assert_eq!(apply(r, "c"), Ruled::Ok("c", ""));
        assert_eq!(apply(r, "d"), Ruled::Ok("d", ""));
        assert_eq!(apply(r, "e"), Ruled::Err(()));

        let r = super::char_range('b'..'d');
        assert_eq!(apply(r, "a"), Ruled::Err(()));
        assert_eq!(apply(r, "b"), Ruled::Ok("b", ""));
        assert_eq!(apply(r, "c"), Ruled::Ok("c", ""));
        assert_eq!(apply(r, "d"), Ruled::Err(()));

        let r = super::char_range(..);
        assert_eq!(apply(r, "a"), Ruled::Ok("a", ""));
        assert_eq!(apply(r, "e"), Ruled::Ok("e", ""));
    }
}