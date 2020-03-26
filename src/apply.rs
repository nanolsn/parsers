use super::ruled::Ruled;

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

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err>;
}

impl<'i> Apply<&'i str> for char {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.starts_with(*self) {
            input.split_at(self.len_utf8()).into()
        } else {
            Ruled::Err(())
        }
    }
}

impl<'i> Apply<&'i str> for &str {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.starts_with(self) {
            input.split_at(self.len()).into()
        } else {
            Ruled::Err(())
        }
    }
}

impl<'i> Apply<&'i str> for String {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        self.as_str().apply(input)
    }
}

impl<F, I, R, E> Apply<I> for F
    where
        F: Fn(I) -> Ruled<I, R, E>,
{
    type Err = E;
    type Res = R;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { self(input) }
}

impl<I> Apply<I> for () {
    type Err = ();
    type Res = ();

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { Ruled::Ok((), input) }
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
        assert_eq!(apply(r, "hi!"), Ruled::Err(()));
    }

    #[test]
    fn char() {
        let r = rule('@');
        assert_eq!(apply(r, &*"@#".to_owned()), Ruled::Ok("@", "#"));
        assert_eq!(apply(r, "$"), Ruled::Err(()));
    }

    //noinspection RsBorrowChecker
    #[test]
    fn tuple() {
        let r = (rule('@'), '#', "__");
        assert_eq!(apply(r, "@#__"), Ruled::Ok(("@", "#", "__"), ""));
        assert_eq!(apply(r, "@#!"), Ruled::Err(()));
        assert_eq!(apply(r, "#$"), Ruled::Err(()));

        let r = (rule('0').map(|_| 0), '1', "23", "4");
        assert_eq!(apply(r, "012345"), Ruled::Ok((0, "1", "23", "4"), "5"));
        assert_eq!(apply(r, "0123"), Ruled::Err(()));
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
    fn dyn_rules() {
        type StrApply<'c> = dyn Apply<&'c str, Res=&'c str, Err=()>;

        fn apply_dyn<'c>(code: &'c str, rule: &StrApply<'c>) -> Result<&'c str, ()> {
            rule.apply(code).result()
        }

        let a = rule('q');
        let b = rule("w");
        let rules: Vec<&StrApply> = vec![&a, &b];
        let results: Vec<Result<&str, ()>> = rules
            .into_iter()
            .map(|rule| apply_dyn("q", rule))
            .collect();

        assert_eq!(results, vec![Ok("q"), Err(())])
    }
}
