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

impl<I, T> Apply<I> for &T
    where
        T: Apply<I> + ?Sized,
{
    type Err = T::Err;
    type Res = T::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { (*self).apply(input) }
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

impl<'i> Apply<&'i str> for str {
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
