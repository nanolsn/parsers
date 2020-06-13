use crate::{
    prelude::*,
    IntoRule,
    compound::{Cat, Or, Fst, Snd, Not},
};

/// The wrapper to provide useful features of [rules].
///
/// Use the [`rul`] function, that is a constructor of `Rul`, to wrap your type.
///
/// Why can it be useful:
///
/// * Use rule operators with built-in types.
/// * Automatically convert types that don't implement [`Rule`] to types that do.
/// * Prevent accidental calling a method with the same name on the target type
///   instead of calling a [trait] method.
///
/// # Example
///
/// Call the required method:
/// ```
/// # // This is a fake code to pass doc tests. I'm annoyed by "ignored: 1" message.
/// # struct Fake<T>(T);
/// # impl<T> Fake<T> {
/// #     fn or(self, _: Result<i32, ()>) {}
/// #     fn into_inner(self) -> T { self.0 }
/// # };
/// # let rul = |a| Fake(a);
/// let res: Result<i32, ()> = Ok(1);
///
/// res.or(res); // calls `Result::or` function.
/// rul(res).or(res); // calls `Rule::or` trait function.
/// rul(res).into_inner().or(res); // calls `Result::or` function.
/// ```
///
/// [rules]: ../trait.Rule.html
/// [`rul`]: ./fn.rul.html
/// [`Rule`]: ../trait.Rule.html
/// [trait]: ../trait.Rule.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rul<R>(R);

impl<R> Rul<R> {
    pub fn get(&self) -> &R { &self.0 }

    pub fn get_mut(&mut self) -> &R { &mut self.0 }

    pub fn into_inner(self) -> R { self.0 }
}

impl<'r, I, R> Rule<'r, I> for Rul<R>
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
pub fn rul<'r, I, N, R>(rule: N) -> Rul<R>
    where
        N: IntoRule<R, I>,
        R: Rule<'r, I>,
{ Rul(rule.into_rule()) }

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

impl<R, T> std::ops::Shl<T> for Rul<R> {
    type Output = Fst<R, T>;

    fn shl(self, rhs: T) -> Self::Output { Fst(self.0, rhs) }
}

impl<R, T> std::ops::Shr<T> for Rul<R> {
    type Output = Snd<R, T>;

    fn shr(self, rhs: T) -> Self::Output { Snd(self.0, rhs) }
}

impl<R> std::ops::Not for Rul<R> {
    type Output = Not<R>;

    fn not(self) -> Self::Output { Not(self.0) }
}
