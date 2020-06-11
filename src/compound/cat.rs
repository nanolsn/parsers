use crate::{
    Rule,
    Ruled,
    Concat,
    compound::Or,
};

#[derive(Debug)]
pub struct Cat<A, B, C>(A, B, std::marker::PhantomData<C>);

impl<A, B, C> Cat<A, B, C> {
    pub fn new(a: A, b: B) -> Self { Self(a, b, std::marker::PhantomData) }
}

impl<A, B, C> Clone for Cat<A, B, C>
    where
        A: Clone,
        B: Clone,
{
    fn clone(&self) -> Self { Cat::new(self.0.clone(), self.1.clone()) }
}

impl<A, B, C> Copy for Cat<A, B, C>
    where
        A: Copy,
        B: Copy,
{}

impl<A, B, C> Cat<A, B, C> {
    pub fn cat<'r, I: 'r, R>(self, rhs: R) -> Cat<Self, R, C>
        where
            R: Rule<'r, I>,
    { Cat::new(self, rhs) }
}

impl<'r, I: 'r, A, B, C> Rule<'r, I> for Cat<A, B, C>
    where
        A: Rule<'r, I>,
        B: Rule<'r, I, Exp=A::Exp>,
        C: Concat<A::Mat, B::Mat>,
{
    type Mat = C;
    type Exp = A::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|l, i| self.1.rule(i)
                .map(|r| C::concat(l, r))
            )
    }
}

impl<A, B, C, T> std::ops::BitOr<T> for Cat<A, B, C> {
    type Output = Or<Cat<A, B, C>, T>;

    fn bitor(self, rhs: T) -> Self::Output { Or(self, rhs) }
}

impl<A, B, T> std::ops::BitAnd<T> for Cat<A, B, &'static str> {
    type Output = Cat<Cat<A, B, &'static str>, T, &'static str>;

    fn bitand(self, rhs: T) -> Self::Output { Cat::new(self, rhs) }
}

impl<A, B, T> std::ops::Add<T> for Cat<A, B, String> {
    type Output = Cat<Cat<A, B, String>, T, String>;

    fn add(self, rhs: T) -> Self::Output { Cat::new(self, rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Ruled::*;
    use crate::Failed;

    #[test]
    fn cat() {
        let r = '@'.cat('#');
        assert_eq!(r.rule("@#"), Match("@#".to_owned(), ""));
        assert_eq!(r.rule("@!"), Expected(Failed::Char('#')));
        assert_eq!(r.rule("@"), Expected(Failed::Char('#')));

        let r = "q".cat("w").cat("e");
        assert_eq!(r.rule("qwe"), Match("qwe".to_owned(), ""));
        assert_eq!(r.rule("qwe123"), Match("qwe".to_owned(), "123"));
        assert_eq!(r.rule("123"), Expected(Failed::Str("q")));
    }
}
