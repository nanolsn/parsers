use super::{
    Rule,
    Ruled,
    IntoRule,
};

/// The type wrapper to provide useful methods for a [rule].
///
/// [rule]: ./trait.Rule.html
#[derive(Copy, Clone, Debug)]
pub struct Rul<R>(R);

impl<'r, I: 'r, R> Rule<'r, I> for Rul<R>
    where
        R: Rule<'r, I>,
{
    type Mat = R::Mat;
    type Exp = R::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { self.0.rule(input) }
}

/// Constructor for [`Rul`].
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
