use std::ops::{Range, RangeInclusive};

pub trait Parse<I> {
    type Err;
    type Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err>;

    fn map<F>(self, f: F) -> Map<Self, F>
        where
            Self: Sized,
    {
        Map(self, f)
    }

    fn map_err<F>(self, f: F) -> MapErr<Self, F>
        where
            Self: Sized,
    {
        MapErr(self, f)
    }
}

impl<'i> Parse<&'i str> for str {
    type Err = ();
    type Out = &'i str;

    fn parse<'s>(&'s self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        if input.starts_with(self) {
            let r = input.split_at(self.len());
            Ok(r)
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
            Some(c) if c == *self => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left, right))
            }
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
            Some(c) if self(c) => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left, right))
            }
            _ => Err(()),
        }
    }
}

impl<'i> Parse<&'i str> for Range<char> {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left, right))
            }
            _ => Err(()),
        }
    }
}

impl<'i> Parse<&'i str> for RangeInclusive<char> {
    type Err = ();
    type Out = &'i str;

    fn parse(&self, input: &'i str) -> Result<(Self::Out, &'i str), Self::Err> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => {
                let (left, right) = input.split_at(c.len_utf8());
                Ok((left, right))
            }
            _ => Err(()),
        }
    }
}

impl<P1, P2, I, R1, R2, E> Parse<I> for (P1, P2)
    where
        P1: Parse<I, Out=R1, Err=E>,
        P2: Parse<I, Out=R2, Err=E>,
{
    type Err = E;
    type Out = (R1, R2);

    fn parse(&self, input: I) -> Result<((R1, R2), I), E> {
        self.0.parse(input).and_then(
            |(r1, rest)| self.1.parse(rest).map(
                |(r2, rest)| ((r1, r2), rest)
            )
        )
    }
}

pub struct Map<P, F>(P, F);

impl<P, F, A, B, I> Parse<I> for Map<P, F>
    where
        P: Parse<I, Out=A>,
        F: Fn(A) -> B,
{
    type Err = P::Err;
    type Out = B;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).map(|(out, rest)| ((self.1)(out), rest))
    }
}

pub struct MapErr<P, F>(P, F);

impl<P, F, E, G, I> Parse<I> for MapErr<P, F>
    where
        P: Parse<I, Err=E>,
        F: Fn(E) -> G + Copy,
{
    type Err = G;
    type Out = P::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        self.0.parse(input).map_err(self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

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
    fn parse_map() {
        let l = parser("a").map(|_| 1);

        assert_eq!(l.parse("ab"), Ok((1, "b")));
        assert_eq!(l.parse("c"), Err(()));
    }

    #[test]
    fn parse_map_err() {
        let l = parser("a").map_err(|_| 1);

        assert_eq!(l.parse("ab"), Ok(("a", "b")));
        assert_eq!(l.parse("c"), Err(1));
    }
}
