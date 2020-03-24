use super::{
    apply::Apply,
    ruled::Ruled,
    rules::{
        cat::Cat,
        or::Or,
    },
};

pub fn rule<R, I>(r: R) -> Rule<R>
    where
        R: Apply<I>,
{ Rule(r) }

#[derive(Copy, Clone, Debug)]
pub struct Rule<R>(pub R);

impl<R, I> Apply<I> for Rule<R>
    where
        R: Apply<I>,
{
    type Err = R::Err;
    type Res = R::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.0.apply(input) }
}

impl<R> std::ops::Deref for Rule<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<R> std::ops::DerefMut for Rule<R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<L, R> std::ops::BitAnd<R> for Rule<L> {
    type Output = Rule<Cat<L, R>>;

    fn bitand(self, rhs: R) -> Self::Output { Rule(Cat(self.0, rhs)) }
}

impl<L, R> std::ops::BitOr<R> for Rule<L> {
    type Output = Rule<Or<L, R>>;

    fn bitor(self, rhs: R) -> Self::Output { Rule(Or(self.0, rhs)) }
}
