use std::ops::{Range, RangeInclusive};

pub trait Parse<I> {
    type Err;
    type Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err>;

    fn parse_result(&self, input: I) -> Result<Self::Out, Self::Err> {
        self.parse(input).map(|(r, _)| r)
    }
}

impl<'i> Parse<&'i str> for str {
    type Err = ();
    type Out = &'i str;

    fn parse<'s>(&'s self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        if input.starts_with(self) {
            Ok(input.split_at(self.len()))
        } else {
            Err(())
        }
    }
}

impl<'i> Parse<&'i str> for &str {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        Parse::parse(*self, input)
    }
}

impl<'i> Parse<&'i str> for String {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        Parse::parse(self.as_str(), input)
    }
}

impl<'i> Parse<&'i str> for char {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if c == *self => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'i, F> Parse<&'i str> for F
    where
        F: Fn(char) -> bool,
{
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if self(c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'i> Parse<&'i str> for Range<char> {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'i> Parse<&'i str> for RangeInclusive<char> {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'i, T> Parse<&'i [T]> for [T]
    where
        T: PartialEq + Clone,
{
    type Err = ();
    type Out = &'i [T];

    fn parse<'s>(&'s self, input: &'i [T]) -> Result<(Self::Out, &'i [T]), Self::Err> {
        if input.starts_with(self) {
            Ok(input.split_at(self.len()))
        } else {
            Err(())
        }
    }
}

impl<'i, T> Parse<&'i [T]> for &[T]
    where
        T: PartialEq + Clone,
{
    type Err = ();
    type Out = &'i [T];

    fn parse(&self, input: &'i [T]) -> Result<(Self::Out, &'i [T]), Self::Err> {
        self.as_ref().parse(input)
    }
}

impl<'i, T> Parse<&'i [T]> for Vec<T>
    where
        T: PartialEq + Clone,
{
    type Err = ();
    type Out = &'i [T];

    fn parse(&self, input: &'i [T]) -> Result<(Self::Out, &'i [T]), Self::Err> {
        self.as_slice().parse(input)
    }
}

impl_tuple!(P0, P1; r0, r1);
impl_tuple!(P0, P1, P2; r0, r1, r2);
impl_tuple!(P0, P1, P2, P3; r0, r1, r2, r3);
impl_tuple!(P0, P1, P2, P3, P4; r0, r1, r2, r3, r4);
impl_tuple!(P0, P1, P2, P3, P4, P5; r0, r1, r2, r3, r4, r5);
impl_tuple!(P0, P1, P2, P3, P4, P5, P6; r0, r1, r2, r3, r4, r5, r6);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn parse_str() {
        let t = "a";

        assert_eq!(Parse::parse(t, "a b"), Ok(("a", " b")));
        assert_eq!(Parse::parse(t, "b"), Err(()));

        let empty = "";

        assert_eq!(Parse::parse(empty, "a"), Ok(("", "a")));
        assert_eq!(Parse::parse(empty, ""), Ok(("", "")));
    }

    #[test]
    fn parse_string() {
        let s = String::from("z");

        assert_eq!(Parse::parse(&s, "z_x"), Ok(("z", "_x")));
        assert_eq!(Parse::parse(&s, "c"), Err(()));
    }

    #[test]
    fn parse_char() {
        let f = 'f';

        assert_eq!(f.parse("fg"), Ok(("f", "g")));
        assert_eq!(f.parse("e"), Err(()));
    }

    #[test]
    fn parse_fn() {
        let f = |c| match c {
            '0' => true,
            _ => false,
        };

        assert_eq!(f.parse("01"), Ok(("0", "1")));
        assert_eq!(f.parse("1"), Err(()));
    }

    #[test]
    fn parse_range() {
        let l = 'a'..'c';

        assert_eq!(l.parse("a_"), Ok(("a", "_")));
        assert_eq!(l.parse("b"), Ok(("b", "")));
        assert_eq!(l.parse("c"), Err(()));
        assert_eq!(l.parse("."), Err(()));

        let l = 'a'..='c';

        assert_eq!(l.parse("a"), Ok(("a", "")));
        assert_eq!(l.parse("b"), Ok(("b", "")));
        assert_eq!(l.parse("c"), Ok(("c", "")));
        assert_eq!(l.parse("d"), Err(()));
    }

    #[test]
    fn parse_slice() {
        let t = [1, 2];

        assert_eq!(t.as_ref().parse(&[1, 2, 3]), Ok(([1, 2].as_ref(), [3].as_ref())));
        assert_eq!(t.as_ref().parse(&[5]), Err(()));

        let empty = [];

        assert_eq!(empty.as_ref().parse(&[1]), Ok(([].as_ref(), [1].as_ref())));
        assert_eq!(empty.as_ref().parse(&[]), Ok(([].as_ref(), [].as_ref())));
    }

    #[test]
    fn parse_vec() {
        let s = vec![0];

        assert_eq!(s.parse(&[0, 4, 5]), Ok(([0].as_ref(), [4, 5].as_ref())));
        assert_eq!(s.parse(&[5]), Err(()));
    }

    #[test]
    fn parse_tuple() {
        let t = (par("0"), "1");

        assert_eq!(t.parse("0123"), Ok((("0", "1"), "23")));
        assert_eq!(t.parse("0!"), Err(()));

        let t = (par("0").map(|_| 0), "1", "2");

        assert_eq!(t.parse("0123"), Ok(((0, "1", "2"), "3")));
        assert_eq!(t.parse("01"), Err(()));

        let t = (par("0").map(|_| true), "1", "2", "3");

        assert_eq!(t.parse("0123"), Ok(((true, "1", "2", "3"), "")));
        assert_eq!(t.parse("012"), Err(()));

        let t = (par("0"), "1", "2", "3", "4");

        assert_eq!(t.parse("01234"), Ok((("0", "1", "2", "3", "4"), "")));
        assert_eq!(t.parse("0123"), Err(()));

        let t = (par("0"), "1", "2", "3", "4", "5");

        assert_eq!(t.parse("012345"), Ok((("0", "1", "2", "3", "4", "5"), "")));
        assert_eq!(t.parse("01234"), Err(()));

        let t = (par("0"), "1", "2", "3", "4", "5", "6");

        assert_eq!(t.parse("0123456"), Ok((("0", "1", "2", "3", "4", "5", "6"), "")));
        assert_eq!(t.parse("012345"), Err(()));
    }
}
