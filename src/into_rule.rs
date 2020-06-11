use super::Rule;

pub trait IntoRule<R, I> {
    fn into_rule(self) -> R;
}

impl<'r, I: 'r, R> IntoRule<R, I> for R
    where
        R: Rule<'r, I>,
{
    fn into_rule(self) -> R { self }
}
