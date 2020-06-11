use super::{
    Concat,
    Ruled::{self, *},
    Failed,
    compound::{Cat, Or},
};

pub trait Rule<'r, I: 'r> {
    type Mat;
    type Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp>;

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
}

impl<'r, I: 'r, T> Rule<'r, I> for &T
    where
        T: Rule<'r, I> + ?Sized,
{
    type Mat = T::Mat;
    type Exp = T::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { (*self).rule(input) }
}

impl<'r, 'i: 'r> Rule<'r, &'i str> for char {
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

impl<'r, 'i: 'r> Rule<'r, &'i str> for str {
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

impl<'r, 'i: 'r> Rule<'r, &'i str> for String {
    type Mat = &'i str;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        self.as_str().rule(input)
    }
}

impl<'r, I: 'r, R, E> Rule<'r, I> for Result<R, E>
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

impl<'r, I: 'r> Rule<'r, I> for () {
    type Mat = ();
    type Exp = Failed<'r>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { Match((), input) }
}

impl<'r, 'i: 'r, T> Rule<'r, &'i [T]> for [T]
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

impl<'r, 'i: 'r, T> Rule<'r, &'i [T]> for Vec<T>
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
        let ok: Result<_, ()> = Ok(1);
        assert_eq!(ok.rule("!"), Match(1, "!"));

        let err: Result<(), _> = Err(1);
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
}
