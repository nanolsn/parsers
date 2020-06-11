use super::{
    Concat,
    Ruled::{self, *},
    SomeOf,
    compound::*,
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
    type Exp = char;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        if input.starts_with(*self) {
            input.split_at(self.len_utf8()).into()
        } else {
            Expected(*self)
        }
    }
}

impl<'r, 'i: 'r> Rule<'r, &'i str> for str {
    type Mat = &'i str;
    type Exp = &'r str;

    fn rule(&'r self, input: &'i str) -> Ruled<&'i str, Self::Mat, Self::Exp> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Expected(self)
        }
    }
}

impl<'r, 'i: 'r> Rule<'r, &'i str> for String {
    type Mat = &'i str;
    type Exp = &'r str;

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
    type Exp = SomeOf<'static>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { Match((), input) }
}

impl<'r, 'i: 'r, T> Rule<'r, &'i [T]> for [T]
    where
        T: PartialEq,
{
    type Mat = &'i [T];
    type Exp = ();

    fn rule(&'r self, input: &'i [T]) -> Ruled<&'i [T], Self::Mat, Self::Exp> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Expected(())
        }
    }
}

impl<'r, 'i: 'r, T> Rule<'r, &'i [T]> for Vec<T>
    where
        T: PartialEq,
{
    type Mat = &'i [T];
    type Exp = ();

    fn rule(&'r self, input: &'i [T]) -> Ruled<&'i [T], Self::Mat, Self::Exp> {
        self.as_slice().rule(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rul::rul;

    #[test]
    fn string() {
        let hello = "hello".to_string();
        let r = rul(hello);
        assert_eq!(r.clone().rule("hello!"), Match("hello", "!"));
        assert_eq!(r.rule("hi!"), Expected("hello"));
    }

    #[test]
    fn char() {
        let r = rul('@');
        assert_eq!(r.rule(&*"@#".to_owned()), Match("@", "#"));
        assert_eq!(r.rule("$"), Expected('@'));
    }

    #[test]
    fn result() {
        let ok: Result<_, ()> = Ok(1);
        assert_eq!(ok.rule("!"), Match(1, "!"));

        let err: Result<(), _> = Err(1);
        assert_eq!(err.rule("!"), Expected(1));
    }

    // #[test]
    // fn tuple() {
    //     let r = (rul('@'), '#', "__");
    //     assert_eq!(r.rule("@#__"), Match(("@", "#", "__"), ""));
    //     assert_eq!(r.rule("@#!"), Expected(SomeOf::Str("__")));
    //     assert_eq!(r.rule("#$"), Expected(SomeOf::Char('@')));
    //
    //     let r = (rul('0').map(|_| 0), '1', "23", "4");
    //     assert_eq!(r.rule("012345"), Match((0, "1", "23", "4"), "5"));
    //     assert_eq!(r.rule("0123"), Expected(SomeOf::Str("4")));
    // }

    #[test]
    fn slice() {
        let r = rul(vec![1, 2]);
        assert_eq!(r.rule([1, 2, 3].as_ref()), Match([1, 2].as_ref(), [3].as_ref()));
    }
}
