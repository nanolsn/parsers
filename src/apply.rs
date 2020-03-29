use super::{
    ruled::Ruled,
    expected::Expected,
};

pub fn apply<R, I>(rule: R, input: I) -> Ruled<I, R::Res, R::Err>
    where
        R: Apply<I>,
{ rule.apply(input) }

pub fn apply_result<R, I>(rule: R, input: I) -> Result<R::Res, R::Err>
    where
        R: Apply<I>,
{ apply(rule, input).into() }

pub trait Apply<I> {
    type Err;
    type Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err>;
}

impl<'i> Apply<&'i str> for char {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.starts_with(self) {
            input.split_at(self.len_utf8()).into()
        } else {
            Ruled::Err(Expected::Char(self))
        }
    }
}

impl<'i, 'r> Apply<&'i str> for &'r str {
    type Err = Expected<'r>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Ruled::Err(Expected::Str(self))
        }
    }
}

impl<'i> Apply<&'i str> for String {
    type Err = Expected<'static>;
    type Res = &'i str;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match (&*self).apply(input) {
            Ruled::Ok(r, i) => Ruled::Ok(r, i),
            Ruled::Err(_) => Ruled::Err(Expected::String(self)),
        }
    }
}

impl<F, I, R, E> Apply<I> for F
    where
        F: Fn(I) -> Ruled<I, R, E>,
{
    type Err = E;
    type Res = R;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> { self(input) }
}

impl<I> Apply<I> for () {
    type Err = Expected<'static>;
    type Res = ();

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Ok((), input) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::rule;

    #[test]
    fn string() {
        let hello = "hello".to_string();
        let r = rule(hello);
        assert_eq!(apply(r.clone(), "hello!"), Ruled::Ok("hello", "!"));
        assert_eq!(apply(r, "hi!"), Ruled::Err(Expected::String("hello".to_owned())));
    }

    #[test]
    fn char() {
        let r = rule('@');
        assert_eq!(apply(r, &*"@#".to_owned()), Ruled::Ok("@", "#"));
        assert_eq!(apply(r, "$"), Ruled::Err(Expected::Char('@')));
    }

    //noinspection RsBorrowChecker
    #[test]
    fn tuple() {
        let r = (rule('@'), '#', "__");
        assert_eq!(apply(r, "@#__"), Ruled::Ok(("@", "#", "__"), ""));
        assert_eq!(apply(r, "@#!"), Ruled::Err(Expected::Str("__")));
        assert_eq!(apply(r, "#$"), Ruled::Err(Expected::Char('@')));

        let r = (rule('0').map(|_| 0), '1', "23", "4");
        assert_eq!(apply(r, "012345"), Ruled::Ok((0, "1", "23", "4"), "5"));
        assert_eq!(apply(r, "0123"), Ruled::Err(Expected::Str("4")));
    }

    #[test]
    fn func() {
        let f = |s| match s {
            "foo" => Ruled::Ok("ok", s),
            "test" => Ruled::Ok(s, s),
            _ => Ruled::Err(()),
        };

        assert_eq!(apply(f, "foo"), Ruled::Ok("ok", "foo"));
        assert_eq!(apply(f, "test"), Ruled::Ok("test", "test"));
        assert_eq!(apply(f, "bar"), Ruled::Err(()));
    }
}
