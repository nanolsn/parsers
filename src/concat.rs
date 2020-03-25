pub trait Concat<L, R>
    where
        Self: Sized,
{
    fn empty() -> Self;

    fn concat(l: L, r: R) -> Self;
}

impl<L, R> Concat<L, R> for String
    where
        L: Into<String>,
        R: AsRef<str>,
{
    fn empty() -> Self { String::new() }

    fn concat(l: L, r: R) -> Self {
        let mut res = l.into();
        res.push_str(r.as_ref());
        res
    }
}

impl<T> Concat<Vec<T>, T> for Vec<T> {
    fn empty() -> Self { Vec::new() }

    fn concat(mut l: Vec<T>, r: T) -> Self {
        l.push(r);
        l
    }
}
