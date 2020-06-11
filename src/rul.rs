use super::{
    Rule,
    Ruled,
    IntoRule,
    compound::{Cat, Or},
};

/// The wrapper to provide useful features of [rules].
///
/// Use the [`rul`] function, that is a constructor of `Rul`, to wrap your type.
///
/// Why can it be useful:
///
/// * Use rule operators with built-in types.
/// * Automatically convert types that don't implement [`Rule`] to types that do.
///
/// [rules]: ./trait.Rule.html
/// [`rul`]: ./fn.rul.html
/// [`Rule`]: ./trait.Rule.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rul<R>(R);

impl<'r, I: 'r, R> Rule<'r, I> for Rul<R>
    where
        R: Rule<'r, I>,
{
    type Mat = R::Mat;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { self.0.rule(input) }
}

/// The constructor of [`Rul`].
///
/// [`Rul`]: ./struct.Rul.html
pub fn rul<'r, I: 'r, N, R>(rule: N) -> Rul<R>
    where
        N: IntoRule<R, I>,
        R: Rule<'r, I>,
{ Rul(rule.into_rule()) }

impl<R> std::ops::Deref for Rul<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<R> std::ops::DerefMut for Rul<R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<R, T> std::ops::BitOr<T> for Rul<R> {
    type Output = Or<R, T>;

    fn bitor(self, rhs: T) -> Self::Output { Or(self.0, rhs) }
}

impl<R, T> std::ops::BitAnd<T> for Rul<R> {
    type Output = Cat<R, T, &'static str>;

    fn bitand(self, rhs: T) -> Self::Output { Cat::new(self.0, rhs) }
}

impl<R, T> std::ops::Add<T> for Rul<R> {
    type Output = Cat<R, T, String>;

    fn add(self, rhs: T) -> Self::Output { Cat::new(self.0, rhs) }
}
