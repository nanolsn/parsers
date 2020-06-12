use crate::{
    prelude::*,
    IsEmpty,
};

#[derive(Copy, Clone, Debug)]
pub struct End<R>(pub R);

impl<'r, I: 'r, R> Rule<'r, I> for End<R>
    where
        R: Rule<'r, I>,
        R::Exp: Into<Failed<'r>>,
        I: IsEmpty,
{
    type Mat = R::Mat;
    type Exp = Failed<'r>;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        match self.0.rule(input) {
            Match(r, i) => if i.is_empty() {
                Match(r, i)
            } else {
                Expected(Failed::End)
            },
            Expected(e) => Expected(e.into()),
        }
    }
}

impl_ops!(End<M>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end() {
        let r = 'a'.end();
        assert_eq!(r.rule("a"), Match("a", ""));
        assert_eq!(r.rule("aa"), Expected(Failed::End));
    }
}
