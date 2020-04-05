use super::{
    ruled::Ruled,
    expected::Expected,
};

/// Applies the input to the [rule] and returns [`Ruled`] struct.
///
/// [`Ruled`]: ./enum.Ruled.html
/// [rule]: ./trait.Apply.html
pub fn apply<R, I>(rule: R, input: I) -> Ruled<I, R::Res, R::Err>
    where
        R: Apply<I>,
{ rule.apply(input) }

/// Applies the input to the [rule] and returns `Result`.
///
/// [rule]: ./trait.Apply.html
pub fn apply_result<R, I>(rule: R, input: I) -> Result<R::Res, R::Err>
    where
        R: Apply<I>,
{ apply(rule, input).into() }

/// An interface of rules.
///
/// Something that implements this trait is the rule.
/// Actually, this trait could be called `Rule`, but it is called as a function of application.
///
/// It takes a rule by value and some generalized input `I`.
/// As a result the [`apply`] function returns [`Ruled`].
///
/// [`apply`]: ./trait.Apply.html#tymethod.apply
/// [`Ruled`]: ./enum.Ruled.html
pub trait Apply<I> {
    /// The type of the error.
    ///
    /// It usually contains information about the expected value.
    /// Contained in the [`Ruled`] structure.
    ///
    /// [`Ruled`]: ./enum.Ruled.html
    type Err;

    /// The type of the result.
    ///
    /// It is returned when the application of the rules was successful.
    /// Contained in the [`Ruled`] structure.
    ///
    /// [`Ruled`]: ./enum.Ruled.html
    type Res;

    /// Applies the rule to input and returns a result.
    ///
    /// If the application of the rules is successful, returns [`Ruled::Ok(Result, Input)`].
    /// Otherwise, when the application is unsuccessful, returns [`Ruled::Err(Error)`].
    ///
    /// [`Ruled::Ok(Result, Input)`]: ./enum.Ruled.html#variant.Ok
    /// [`Ruled::Err(Error)`]: ./enum.Ruled.html#variant.Err
    ///
    /// # Examples
    ///
    /// ```
    /// # use parsers::{Apply, Ruled, Expected};
    /// // Match letter `A`
    /// let rule = 'A';
    ///
    /// let ok = "A.";
    /// let fail = "B";
    ///
    /// // It returns the `Ok` result, since the input starts with `A`.
    /// // The result contains parts of the matched input and the remaining input.
    /// assert_eq!(Ruled::Ok("A", "."), rule.apply(ok));
    ///
    /// // It fails. As an error, it returns the expected value.
    /// assert_eq!(Ruled::Err(Expected::Char('A')), rule.apply(fail));
    /// ```
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
    type Res = String;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        match (&*self).apply(input) {
            Ruled::Ok(_, i) => Ruled::Ok(self, i),
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

impl<I, R, E> Apply<I> for Result<R, E> {
    type Err = E;
    type Res = R;

    fn apply(self, input: I) -> Ruled<I, R, E> {
        match self {
            Ok(o) => Ruled::Ok(o, input),
            Err(e) => Ruled::Err(e),
        }
    }
}

impl<I> Apply<I> for () {
    type Err = Expected<'static>;
    type Res = ();

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Ok((), input) }
}

impl<'i, 'r, T> Apply<&'i [T]> for &'r [T]
    where
        T: PartialEq,
{
    type Err = ();
    type Res = &'i [T];

    fn apply(self, input: &'i [T]) -> Ruled<&'i [T], Self::Res, Self::Err> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Ruled::Err(())
        }
    }
}

impl<'i, T> Apply<&'i [T]> for Vec<T>
    where
        T: PartialEq,
{
    type Err = ();
    type Res = &'i [T];

    fn apply(self, input: &'i [T]) -> Ruled<&'i [T], Self::Res, Self::Err> {
        self.as_slice().apply(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::rule;

    #[test]
    fn string() {
        let hello = "hello".to_string();
        let r = rule(hello);
        assert_eq!(apply(r.clone(), "hello!"), Ruled::Ok("hello".to_owned(), "!"));
        assert_eq!(apply(r, "hi!"), Ruled::Err(Expected::String("hello".to_owned())));
    }

    #[test]
    fn char() {
        let r = rule('@');
        assert_eq!(apply(r, &*"@#".to_owned()), Ruled::Ok("@", "#"));
        assert_eq!(apply(r, "$"), Ruled::Err(Expected::Char('@')));
    }

    #[test]
    fn result() {
        let r = rule::<Result<i64, ()>, &str>(Ok(1));
        assert_eq!(apply(r, "!"), Ruled::Ok(1, "!"));

        let r = rule::<Result<(), i64>, &str>(Err(1));
        assert_eq!(apply(r, "!"), Ruled::Err(1));
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

    #[test]
    fn slice() {
        let r = rule(vec![1, 2]);
        assert_eq!(apply(r, [1, 2, 3].as_ref()), Ruled::Ok([1, 2].as_ref(), [3].as_ref()));
    }
}
