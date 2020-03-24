#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ruled<I, R, E = ()> {
    Ok(R, I),
    Err(E),
}

impl<I, R, E> Ruled<I, R, E> {
    pub fn is_ok(&self) -> bool {
        match self {
            Ruled::Ok(_, _) => true,
            Ruled::Err(_) => false,
        }
    }

    pub fn is_err(&self) -> bool { !self.is_ok() }

    pub fn ok(self) -> Option<R> {
        match self {
            Ruled::Ok(r, _) => Some(r),
            Ruled::Err(_) => None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            Ruled::Ok(_, _) => None,
            Ruled::Err(e) => Some(e),
        }
    }

    pub fn map<F, K>(self, f: F) -> Ruled<I, K, E>
        where
            F: FnOnce(R) -> K,
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(f(r), i),
            Ruled::Err(e) => Ruled::Err(e),
        }
    }

    pub fn map_err<F, Q>(self, f: F) -> Ruled<I, R, Q>
        where
            F: FnOnce(E) -> Q,
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(r, i),
            Ruled::Err(e) => Ruled::Err(f(e)),
        }
    }

    pub fn and_then<F, J, K>(self, f: F) -> Ruled<J, K, E>
        where
            F: FnOnce(R, I) -> Ruled<J, K, E>
    {
        match self {
            Ruled::Ok(r, i) => f(r, i),
            Ruled::Err(e) => Ruled::Err(e),
        }
    }

    pub fn or_else<F>(self, f: F) -> Ruled<I, R, E>
        where
            F: FnOnce(E) -> Ruled<I, R, E>
    {
        match self {
            Ruled::Ok(r, i) => Ruled::Ok(r, i),
            Ruled::Err(e) => f(e),
        }
    }

    pub fn result(self) -> Result<R, E> {
        match self {
            Ruled::Ok(ok, _) => Ok(ok),
            Ruled::Err(err) => Err(err),
        }
    }
}

impl<I, R, E> From<(R, I)> for Ruled<I, R, E> {
    fn from((l, r): (R, I)) -> Self { Ruled::Ok(l, r) }
}

impl<I, R, E> Into<Result<R, E>> for Ruled<I, R, E> {
    fn into(self) -> Result<R, E> { self.result() }
}
