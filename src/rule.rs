use super::{
    prelude::*,
    Concat,
    IsEmpty,
    compound::*,
};

pub trait Rule<'r, I> {
    type Mat;
    type Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp>;

    fn test(&'r self, input: I) -> bool
        where
            I: IsEmpty,
    {
        match self.rule(input) {
            Match(_, i) if i.is_empty() => true,
            _ => false,
        }
    }

    fn cat<C, R>(self, rhs: R) -> Cat<Self, R, C>
        where
            R: Rule<'r, I, Exp=Self::Exp>,
            C: Concat<Self::Mat, R::Mat>,
            Self: Sized,
    { Cat::new(self, rhs) }

    fn or<R>(self, rhs: R) -> Or<Self, R>
        where
            R: Rule<'r, I, Exp=Self::Exp>,
            Self: Sized,
    { Or(self, rhs) }

    fn fst<R>(self, rhs: R) -> Fst<Self, R>
        where
            R: Rule<'r, I>,
            R::Exp: Into<Self::Exp>,
            Self: Sized,
    { Fst(self, rhs) }

    fn snd<R>(self, rhs: R) -> Snd<Self, R>
        where
            R: Rule<'r, I>,
            Self::Exp: Into<R::Exp>,
            Self: Sized,
    { Snd(self, rhs) }

    fn filter<F>(self, f: F) -> Filter<Self, F>
        where
            F: Fn(&Self::Mat) -> bool,
            Self::Exp: Into<Failed<'r>>,
            Self: Sized,
    { Filter(self, f) }

    fn to<T>(self) -> To<Self, T>
        where
            Self::Mat: Into<T>,
            Self: Sized,
    { To::new(self) }

    fn map<F, K>(self, f: F) -> Map<Self, F>
        where
            F: Fn(Self::Mat) -> K,
            Self: Sized,
    { Map(self, f) }

    fn map_exp<F, Q>(self, f: F) -> MapExp<Self, F>
        where
            F: Fn(Self::Exp) -> Q,
            Self: Sized,
    { MapExp(self, f) }

    fn not(self) -> Not<Self>
        where
            I: Copy,
            Self: Sized,
    { Not(self) }

    fn opt(self) -> Opt<Self>
        where
            I: Copy,
            Self: Sized,
    { Opt(self) }

    fn or_default(self) -> OrDefault<Self>
        where
            I: Copy,
            Self::Mat: Default,
            Self: Sized,
    { OrDefault(self) }

    fn range<C, B>(self, rng: B) -> Range<Self, C>
        where
            B: std::ops::RangeBounds<usize>,
            I: Copy,
            C: Concat<C, Self::Mat>,
            Self: Sized,
    { Range::from_range(self, rng) }

    fn repeat<C>(self, times: usize) -> Range<Self, C>
        where
            I: Copy,
            C: Concat<C, Self::Mat>,
            Self: Sized,
    { Range::from_range(self, times..=times) }

    fn until<C, U>(self, until: U) -> Until<Self, U, C>
        where
            U: Rule<'r, I>,
            I: Copy,
            C: Concat<C, Self::Mat>,
            Self: Sized,
    { Until::new(self, until) }

    fn end(self) -> End<Self>
        where
            Self::Exp: Into<Failed<'r>>,
            I: IsEmpty,
            Self: Sized,
    { End(self) }
}

impl<'r, I, T> Rule<'r, I> for &T
    where
        T: Rule<'r, I> + ?Sized,
{
    type Mat = T::Mat;
    type Exp = T::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { (**self).rule(input) }
}

impl<'r, I, T> Rule<'r, I> for &mut T
    where
        T: Rule<'r, I> + ?Sized,
{
    type Mat = T::Mat;
    type Exp = T::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { (**self).rule(input) }
}

impl<'r, I, T> Rule<'r, I> for Box<T>
    where
        T: Rule<'r, I> + ?Sized,
{
    type Mat = T::Mat;
    type Exp = T::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { (**self).rule(input) }
}

impl<'r, 'i> Rule<'r, &'i str> for char {
    type Mat = &'i str;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        if input.starts_with(*self) {
            input.split_at(self.len_utf8()).into()
        } else {
            Expected(Failed::Char(*self))
        }
    }
}

impl<'r, 'i> Rule<'r, &'i str> for str {
    type Mat = &'i str;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Expected(Failed::Str(self))
        }
    }
}

impl<'r, 'i> Rule<'r, &'i str> for String {
    type Mat = &'i str;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        self.as_str().rule(input)
    }
}

impl<'r, I, R, E> Rule<'r, I> for Result<R, E>
    where
        R: Copy,
        E: Copy,
{
    type Mat = R;
    type Exp = E;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        match self {
            Ok(r) => Match(*r, input),
            Err(e) => Expected(*e),
        }
    }
}

impl<'r, I> Rule<'r, I> for () {
    type Mat = ();
    type Exp = Failed<'r>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { Match((), input) }
}

impl<'r, 'i, T: 'r> Rule<'r, &'i [T]> for [T]
    where
        T: PartialEq,
{
    type Mat = &'i [T];
    type Exp = &'r [T];

    fn rule(&'r self, input: &'i [T]) -> Ruled<&'i [T], Self::Mat, Self::Exp> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Expected(self)
        }
    }
}

impl<'r, 'i, T: 'r> Rule<'r, &'i [T]> for Vec<T>
    where
        T: PartialEq,
{
    type Mat = &'i [T];
    type Exp = &'r [T];

    fn rule(&'r self, input: &'i [T]) -> Ruled<&'i [T], Self::Mat, Self::Exp> {
        self.as_slice().rule(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!("hello".to_string().rule("hello!"), Match("hello", "!"));
        assert_eq!("hello".to_string().rule("hi!"), Expected(Failed::Str("hello")));
    }

    #[test]
    fn char() {
        let r = '@';
        assert_eq!(r.rule(&*"@#".to_owned()), Match("@", "#"));
        assert_eq!(r.rule("$"), Expected(Failed::Char('@')));
    }

    #[test]
    fn result() {
        let ok: Result<i32, ()> = Ok(1);
        assert_eq!(ok.rule("!"), Match(1, "!"));

        let err: Result<(), i32> = Err(1);
        assert_eq!(err.rule("!"), Expected(1));
    }

    #[test]
    fn tuple() {
        let r = ('@', '#', "__");
        assert_eq!(r.rule("@#__"), Match(("@", "#", "__"), ""));
        assert_eq!(r.rule("@#!"), Expected(Failed::Str("__")));
        assert_eq!(r.rule("#$"), Expected(Failed::Char('@')));
    }

    #[test]
    fn slice() {
        let r = vec![1, 2];
        assert_eq!(r.rule([1, 2, 3].as_ref()), Match([1, 2].as_ref(), [3].as_ref()));
    }

    #[test]
    fn ops() {
        use crate::prelude::*;

        let r = rul("a") | "b" | "c";
        let r = r & r & r;
        let r = !!rul("") >> "" >> r << "";
        assert_eq!(r.rule("abc"), Match("abc", ""));
    }
}
