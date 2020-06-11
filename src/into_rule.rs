use super::Rule;

/// An interface of type conversion to those that implement [`Rule`].
/// If a type already implements [`Rule`], then the conversion does nothing.
///
/// [`Rule`]: ./trait.Rule.html
pub trait IntoRule<R, I> {
    fn into_rule(self) -> R;
}

impl<'r, I: 'r, R> IntoRule<R, I> for R
    where
        R: Rule<'r, I>,
{
    fn into_rule(self) -> R { self }
}
