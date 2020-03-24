pub trait Concat<R> {
    type Res;

    fn concat(self, r: R) -> Self::Res;
}

impl<L, R> Concat<R> for L
    where
        L: Into<String>,
        R: AsRef<str>,
{
    type Res = String;

    fn concat(self, r: R) -> Self::Res {
        let mut res = self.into();
        res.push_str(r.as_ref());
        res
    }
}
