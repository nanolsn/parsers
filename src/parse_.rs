use std::ops::{Range, RangeInclusive};

pub type Parsed<'p, R, E=(), O=&'p str> = Result<(R, O), E>;

pub trait Parse<'p> {
    type Res: 'p;
    type Err: 'p;
    type On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On>;

    fn parse_result(&self, input: Self::On) -> Result<Self::Res, Self::Err> {
        self.parse(input).map(|(r, _)| r)
    }

    fn parse_unwrap(&self, input: Self::On) -> Self::Res {
        self.parse(input).ok().unwrap().0
    }
}

impl<'p, F, P> Parse<'p> for F
    where
        F: Fn() -> P,
        P: Parse<'p>,
{
    type Res = P::Res;
    type Err = P::Err;
    type On = P::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self().parse(input)
    }
}

impl<'p> Parse<'p> for str {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        if input.starts_with(self) {
            Ok(input.split_at(self.len()))
        } else {
            Err(())
        }
    }
}

impl<'p> Parse<'p> for &str {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        Parse::parse(*self, input)
    }
}

impl<'p> Parse<'p> for String {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        Parse::parse(self.as_str(), input)
    }
}

impl<'p> Parse<'p> for char {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        match input.chars().next() {
            Some(c) if c == *self => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PredFn<F>(pub(crate) F);

impl<'p, F> Parse<'p> for PredFn<F>
    where
        F: Fn(char) -> bool,
{
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) ->Parsed<Self::Res, Self::Err, Self::On> {
        match input.chars().next() {
            Some(c) if (self.0)(c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'p> Parse<'p> for Range<char> {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'p> Parse<'p> for RangeInclusive<char> {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        match input.chars().next() {
            Some(c) if self.contains(&c) => Ok(input.split_at(c.len_utf8())),
            _ => Err(()),
        }
    }
}

impl<'p, T> Parse<'p> for [T]
    where
        T: PartialEq + Clone + 'p,
{
    type Res = &'p [T];
    type Err = ();
    type On = &'p [T];

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        if input.starts_with(self) {
            Ok(input.split_at(self.len()))
        } else {
            Err(())
        }
    }
}

impl<'p, T> Parse<'p> for &[T]
    where
        T: PartialEq + Clone + 'p,
{
    type Res = &'p [T];
    type Err = ();
    type On = &'p [T];

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.as_ref().parse(input)
    }
}

impl<'p, T> Parse<'p> for Vec<T>
    where
        T: PartialEq + Clone + 'p,
{
    type Res = &'p [T];
    type Err = ();
    type On = &'p [T];

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.as_slice().parse(input)
    }
}

impl<'p> Parse<'p> for () {
    type Res = String;
    type Err = ();
    type On = &'p str;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        Ok((String::new(), input))
    }
}

impl<'p, P1> Parse<'p> for (P1,)
    where
        P1: Parse<'p>,
{
    type Res = P1::Res;
    type Err = P1::Err;
    type On = P1::On;

    fn parse(&self, input: Self::On) -> Parsed<Self::Res, Self::Err, Self::On> {
        self.0.parse(input)
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
    use crate::{par, stringed_par, BoxedParser};

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
        let f = PredFn(|c| match c {
            '0' => true,
            _ => false,
        });

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
    fn parse_unit() {
        assert_eq!(
            ().parse("hello"),
            Ok(("".to_string(), "hello")),
        );
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

    #[test]
    fn dyn_test() {
        let p = par("hello");
        let d: &dyn Parse<Res=&str, Err=(), On=&str> = &p;

        assert_eq!(d.parse("hello!"), Ok(("hello", "!")));

        let b: Box<dyn Parse<Res=&str, Err=(), On=&str>> = Box::new(p);

        assert_eq!(b.parse("hello!"), Ok(("hello", "!")));
    }

    #[test]
    fn fn_test() {
        fn dots<'p>() -> BoxedParser<'p, String> {
            let p = stringed_par('.') & (stringed_par('!') | dots);
            p.boxed()
        }

        let s = "...!".to_string();
        assert_eq!(dots.parse(&s), Ok(("...!".to_string(), "")));
    }
}
