/// The result of applying the rules.
///
/// It is [`Ruled::Ok(R, I)`] when the application of the rules was successful.
/// Then it contains a value of type `R` and the remaining input of type `I`.
///
/// If this is [`Ruled::Err(E)`], then the application failed.
/// It contains an error information of type `E`.
///
/// [`Ruled::Ok(R, I)`]: ./enum.Ruled.html#variant.Ok
/// [`Ruled::Err(E)`]: ./enum.Ruled.html#variant.Err
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ruled<I, R, E> {
    /// Contains the obtained value and the remaining input.
    Ok(R, I),

    /// Contains the error value. Usually contains information about the expected value.
    Err(E),
}

impl<I, R, E> Ruled<I, R, E> {
    /// Constructor from the `Result<R, E>` and an input.
    pub fn from_result(result: Result<R, E>, input: I) -> Self {
        match result {
            Ok(o) => Ruled::Ok(o, input),
            Err(e) => Ruled::Err(e),
        }
    }

    /// Returns `true` if the result is [`Ruled::Ok`].
    ///
    /// [`Ruled::Ok`]: ./enum.Ruled.html#variant.Ok
    pub fn is_ok(&self) -> bool {
        match self {
            Ruled::Ok(_, _) => true,
            Ruled::Err(_) => false,
        }
    }

    /// Returns `true` if the result is [`Ruled::Err`].
    ///
    /// [`Ruled::Err`]: ./enum.Ruled.html#variant.Err
    pub fn is_err(&self) -> bool { !self.is_ok() }

    /// Converts from [`Ruled<I, R, E>`] to `Option<R>`.
    ///
    /// Converts self into an `Option<R>`, consuming self,
    /// and discarding the error, if any.
    ///
    /// [`Ruled<I, R, E>`]: ./enum.Ruled.html
    pub fn ok(self) -> Option<R> {
        match self {
            Ruled::Ok(r, _) => Some(r),
            Ruled::Err(_) => None,
        }
    }

    /// Converts from [`Ruled<I, R, E>`] to `Option<E>`.
    ///
    /// Converts self into an `Option<E>`, consuming self,
    /// and discarding the success value, if any.
    ///
    /// [`Ruled<I, R, E>`]: ./enum.Ruled.html
    pub fn err(self) -> Option<E> {
        match self {
            Ruled::Ok(_, _) => None,
            Ruled::Err(e) => Some(e),
        }
    }

    /// Maps a [`Ruled<I, R, E>`] to [`Ruled<I, K, E>`] by applying a function to
    /// a contained [`Ruled::Ok(R, I)`] value, leaving an input
    /// and an [`Ruled::Err(E)`] untouched.
    ///
    /// [`Ruled<I, R, E>`]: ./enum.Ruled.html
    /// [`Ruled<I, K, E>`]: ./enum.Ruled.html
    /// [`Ruled::Ok(R, I)`]: ./enum.Ruled.html#variant.Ok
    /// [`Ruled::Err(E)`]: ./enum.Ruled.html#variant.Err
    pub fn map<F, K>(self, f: F) -> Ruled<I, K, E>
        where
            F: FnOnce(R) -> K,
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(f(r), i),
            Ruled::Err(e) => Ruled::Err(e),
        }
    }

    /// Maps a [`Ruled<I, R, E>`] to [`Ruled<I, R, Q>`] by applying a function to
    /// a contained [`Ruled::Err(E)`] error, leaving an [`Ruled::Ok(R, I)`] untouched.
    ///
    /// [`Ruled<I, R, E>`]: ./enum.Ruled.html
    /// [`Ruled<I, R, Q>`]: ./enum.Ruled.html
    /// [`Ruled::Ok(R, I)`]: ./enum.Ruled.html#variant.Ok
    /// [`Ruled::Err(E)`]: ./enum.Ruled.html#variant.Err
    pub fn map_err<F, Q>(self, f: F) -> Ruled<I, R, Q>
        where
            F: FnOnce(E) -> Q,
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(r, i),
            Ruled::Err(e) => Ruled::Err(f(e)),
        }
    }

    /// Calls `f` if the result is [`Ruled::Ok`],
    /// otherwise returns the [`Ruled::Err`] value of self.
    ///
    /// [`Ruled::Ok`]: ./enum.Ruled.html#variant.Ok
    /// [`Ruled::Err`]: ./enum.Ruled.html#variant.Err
    pub fn and_then<F, J, K>(self, f: F) -> Ruled<J, K, E>
        where
            F: FnOnce(R, I) -> Ruled<J, K, E>
    {
        match self {
            Ruled::Ok(r, i) => f(r, i),
            Ruled::Err(e) => Ruled::Err(e),
        }
    }

    /// Calls `f` if the result is [`Ruled::Err`],
    /// otherwise returns the [`Ruled::Ok`] value of self.
    ///
    /// [`Ruled::Ok`]: ./enum.Ruled.html#variant.Ok
    /// [`Ruled::Err`]: ./enum.Ruled.html#variant.Err
    pub fn or_else<F, Q>(self, f: F) -> Ruled<I, R, Q>
        where
            F: FnOnce(E) -> Ruled<I, R, Q>
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(r, i),
            Ruled::Err(e) => f(e),
        }
    }

    /// Converts self to `Result<R, E>`.
    pub fn result(self) -> Result<R, E> {
        match self {
            Ruled::Ok(ok, _) => Ok(ok),
            Ruled::Err(err) => Err(err),
        }
    }
}

impl<I, R, E> From<Ruled<I, Result<R, E>, E>> for Ruled<I, R, E> {
    fn from(ruled: Ruled<I, Result<R, E>, E>) -> Self {
        match ruled {
            Ruled::Ok(r, i) => Ruled::from_result(r, i),
            Ruled::Err(e) => Ruled::Err(e),
        }
    }
}

impl<I, R, E> From<(R, I)> for Ruled<I, R, E> {
    fn from((l, r): (R, I)) -> Self { Ruled::Ok(l, r) }
}

impl<I, R, E> From<Ruled<I, R, E>> for Result<R, E> {
    fn from(f: Ruled<I, R, E>) -> Self { f.result() }
}
