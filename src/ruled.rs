/// The result of applying the rules.
///
/// It's [`Match(M, I)`] when the `rule` matches with input.
/// Then it contains a value of type `M` and the remaining input of type `I`.
///
/// If it's [`Expected(E)`], then the application failed.
/// It contains an expected information of type `E`.
///
/// [`Match(M, I)`]: ./enum.Ruled.html#variant.Match
/// [`Expected(E)`]: ./enum.Ruled.html#variant.Expected
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ruled<I, M, E> {
    /// Contains an obtained value and a remaining input.
    Match(M, I),

    /// Contains an information about expected value.
    Expected(E),
}

use Ruled::*;

impl<I, M, E> Ruled<I, M, E> {
    /// Constructor from the `Result<M, E>` and an input.
    pub fn from_result(result: Result<M, E>, input: I) -> Self {
        match result {
            Ok(o) => Match(o, input),
            Err(e) => Expected(e),
        }
    }

    /// Returns `true` if the result is [`Match`].
    ///
    /// [`Match`]: ./enum.Ruled.html#variant.Match
    pub fn is_match(&self) -> bool {
        match self {
            Match(_, _) => true,
            Expected(_) => false,
        }
    }

    /// Returns `true` if the result is [`Expected`].
    ///
    /// [`Expected`]: ./enum.Ruled.html#variant.Expected
    pub fn is_expected(&self) -> bool { !self.is_match() }

    /// Converts from [`Ruled<I, M, E>`] to `Option<M>`.
    ///
    /// Converts self into an `Option<M>`, consuming self,
    /// and discarding the error, if any.
    ///
    /// [`Ruled<I, M, E>`]: ./enum.Ruled.html
    pub fn mat(self) -> Option<M> {
        match self {
            Match(r, _) => Some(r),
            Expected(_) => None,
        }
    }

    /// Converts from [`Ruled<I, M, E>`] to `Option<E>`.
    ///
    /// Converts self into an `Option<E>`, consuming self,
    /// and discarding the success value, if any.
    ///
    /// [`Ruled<I, M, E>`]: ./enum.Ruled.html
    pub fn exp(self) -> Option<E> {
        match self {
            Match(_, _) => None,
            Expected(e) => Some(e),
        }
    }

    /// Maps a [`Ruled<I, M, E>`] to [`Ruled<I, K, E>`] by applying a function to
    /// a contained [`Match(M, I)`] value, leaving an input
    /// and an [`Expected(E)`] untouched.
    ///
    /// [`Ruled<I, M, E>`]: ./enum.Ruled.html
    /// [`Ruled<I, K, E>`]: ./enum.Ruled.html
    /// [`Match(M, I)`]: ./enum.Ruled.html#variant.Match
    /// [`Expected(E)`]: ./enum.Ruled.html#variant.Expected
    pub fn map<F, K>(self, f: F) -> Ruled<I, K, E>
        where
            F: FnOnce(M) -> K,
    {
        match self {
            Match(r, i) => Match(f(r), i),
            Expected(e) => Expected(e),
        }
    }

    /// Maps a [`Ruled<I, M, E>`] to [`Ruled<I, M, Q>`] by applying a function to
    /// a contained [`Expected(E)`] error, leaving an [`Match(R, I)`] untouched.
    ///
    /// [`Ruled<I, M, E>`]: ./enum.Ruled.html
    /// [`Ruled<I, M, Q>`]: ./enum.Ruled.html
    /// [`Match(M, I)`]: ./enum.Ruled.html#variant.Match
    /// [`Expected(E)`]: ./enum.Ruled.html#variant.Expected
    pub fn map_exp<F, Q>(self, f: F) -> Ruled<I, M, Q>
        where
            F: FnOnce(E) -> Q,
    {
        match self {
            Match(r, i) => Match(r, i),
            Expected(e) => Expected(f(e)),
        }
    }

    /// Calls `f` if the result is [`Match`],
    /// otherwise returns the [`Expected`] value of self.
    ///
    /// [`Match`]: ./enum.Ruled.html#variant.Match
    /// [`Expected`]: ./enum.Ruled.html#variant.Expected
    pub fn and_then<F, J, K>(self, f: F) -> Ruled<J, K, E>
        where
            F: FnOnce(M, I) -> Ruled<J, K, E>
    {
        match self {
            Match(r, i) => f(r, i),
            Expected(e) => Expected(e),
        }
    }

    /// Calls `f` if the result is [`Expected`],
    /// otherwise returns the [`Match`] value of self.
    ///
    /// [`Match]: ./enum.Ruled.html#variant.Match
    /// [`Expected`]: ./enum.Ruled.html#variant.Expected
    pub fn or_else<F, Q>(self, f: F) -> Ruled<I, M, Q>
        where
            F: FnOnce(E) -> Ruled<I, M, Q>
    {
        match self {
            Match(r, i) => Match(r, i),
            Expected(e) => f(e),
        }
    }

    /// Converts self to `Result<M, E>`.
    pub fn result(self) -> Result<M, E> {
        match self {
            Match(ok, _) => Ok(ok),
            Expected(err) => Err(err),
        }
    }
}

impl<I, M, E> From<Ruled<I, Result<M, E>, E>> for Ruled<I, M, E> {
    fn from(ruled: Ruled<I, Result<M, E>, E>) -> Self {
        match ruled {
            Match(r, i) => Ruled::from_result(r, i),
            Expected(e) => Expected(e),
        }
    }
}

impl<I, M, E> From<(M, I)> for Ruled<I, M, E> {
    fn from((l, r): (M, I)) -> Self { Match(l, r) }
}

impl<I, M, E> From<Ruled<I, M, E>> for Result<M, E> {
    fn from(f: Ruled<I, M, E>) -> Self { f.result() }
}
