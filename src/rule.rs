use super::{
    apply::Apply,
    ruled::Ruled,
    rules::{
        cat::Cat,
        or::Or,
        fst::Fst,
        snd::Snd,
        range::Range,
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

impl<L, R> std::ops::Shl<R> for Rule<L> {
    type Output = Rule<Fst<L, R>>;

    fn shl(self, rhs: R) -> Self::Output { Rule(Fst(self.0, rhs)) }
}

impl<L, R> std::ops::Shr<R> for Rule<L> {
    type Output = Rule<Snd<L, R>>;

    fn shr(self, rhs: R) -> Self::Output { Rule(Snd(self.0, rhs)) }
}

impl<R> std::ops::Mul<std::ops::Range<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::Range<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: rhs.start,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> std::ops::Mul<std::ops::RangeInclusive<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::RangeInclusive<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: *rhs.start(),
            to: Some(*rhs.end()),
        })
    }
}

impl<R> std::ops::Mul<std::ops::RangeTo<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::RangeTo<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: Some(rhs.end.saturating_sub(1)),
        })
    }
}

impl<R> std::ops::Mul<std::ops::RangeToInclusive<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::RangeToInclusive<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: Some(rhs.end),
        })
    }
}

impl<R> std::ops::Mul<std::ops::RangeFrom<usize>> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, rhs: std::ops::RangeFrom<usize>) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: rhs.start,
            to: None,
        })
    }
}

impl<R> std::ops::Mul<std::ops::RangeFull> for Rule<R> {
    type Output = Rule<Range<R>>;

    fn mul(self, _: std::ops::RangeFull) -> Self::Output {
        Rule(Range {
            rule: self.0,
            from: 0,
            to: None,
        })
    }
}
