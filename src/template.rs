use crate::{Parse, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Template<T>(pub(crate) T);

impl<T> Template<T>
{
    pub fn include<P, R, I>(self, sub: P) -> Parser<IncludedTemplate<P, T>>
        where
            T: Fn(P) -> R,
            R: Parse<I>,
    {
        Parser(IncludedTemplate(sub, self.0))
    }
}

pub fn template<T, P, R, I>(f: T) -> Template<T>
    where
        T: Fn(P) -> R,
        R: Parse<I>,
{
    Template(f)
}

#[derive(Copy, Clone, Debug)]
pub struct IncludedTemplate<P, T>(pub(crate) P, pub(crate) T);

impl<T, P, R, I> Parse<I> for IncludedTemplate<P, T>
    where
        T: Fn(P) -> R,
        P: Copy,
        R: Parse<I>,
{
    type Err = R::Err;
    type Out = R::Out;

    fn parse(&self, input: I) -> Result<(Self::Out, I), Self::Err> {
        let r = (self.1)(self.0);
        r.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::par;

    #[test]
    fn template() {
        let t = super::template(|a| par('[') >> a << ']');
        let p = t.include("hello");

        assert_eq!(p.parse("[hello]").unwrap(), ("hello", ""));
    }
}
