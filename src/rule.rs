use super::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
    rules::{
        cat::Cat,
        or::Or,
        not::Not,
        map::Map,
        map_err::MapErr,
        fst::Fst,
        snd::Snd,
        range::Range,
        and_then::AndThen,
        or_else::OrElse,
        pred::Pred,
        opt::Opt,
        or_default::OrDefault,
        boxed::{BoxedRule, boxed},
    },
};

pub fn rule<R, I>(r: R) -> Rule<R>
    where
        R: Apply<I>,
{ Rule(r) }

#[derive(Copy, Clone, Debug)]
pub struct Rule<R>(pub R);

impl<R> Rule<R> {
    pub fn cat<I, P>(self, rhs: P) -> Rule<Cat<R, P>>
        where
            R: Apply<I>,
            P: Apply<I, Err=R::Err>,
            R::Res: Concat<P::Res>,
    { Rule(Cat(self.0, rhs)) }

    pub fn or<I, P>(self, rhs: P) -> Rule<Or<R, P>>
        where
            R: Apply<I>,
            P: Apply<I, Err=R::Err>,
            I: Copy,
            R::Res: Into<P::Res>,
    { Rule(Or(self.0, rhs)) }

    pub fn not<I>(self) -> Rule<Not<R>>
        where
            R: Apply<I>,
            I: Copy,
    { Rule(Not(self.0)) }

    pub fn map<I, F, K>(self, f: F) -> Rule<Map<R, F>>
        where
            R: Apply<I>,
            F: Fn(R::Res) -> K,
    { Rule(Map(self.0, f)) }

    pub fn map_err<I, F, Q>(self, f: F) -> Rule<MapErr<R, F>>
        where
            R: Apply<I>,
            F: Fn(R::Err) -> Q,
    { Rule(MapErr(self.0, f)) }

    pub fn and_then<I, F, K>(self, f: F) -> Rule<AndThen<R, F>>
        where
            R: Apply<I>,
            F: Fn(R::Res) -> K,
            K: Apply<I, Err=R::Err>,
    { Rule(AndThen(self.0, f)) }

    pub fn or_else<I, F, K>(self, f: F) -> Rule<OrElse<R, F>>
        where
            R: Apply<I>,
            F: Fn(R::Err) -> K,
            K: Apply<I, Res=R::Res>,
            I: Copy,
    { Rule(OrElse(self.0, f)) }

    pub fn pred<I, F>(self, f: F) -> Rule<Pred<R, F>>
        where
            R: Apply<I>,
            F: Fn(&R::Res) -> bool,
    { Rule(Pred(self.0, f)) }

    pub fn opt<I>(self) -> Rule<Opt<R>>
        where
            R: Apply<I>,
            I: Copy,
    { Rule(Opt(self.0)) }

    pub fn or_default<I>(self) -> Rule<OrDefault<R>>
        where
            R: Apply<I>,
            I: Copy,
            R::Res: Default,
    { Rule(OrDefault(self.0)) }

    pub fn boxed<I>(self) -> BoxedRule<I, R::Res, R::Err>
        where
            R: Apply<I> + 'static,
    { boxed(self.0) }
}

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

impl<R> std::ops::Not for Rule<R> {
    type Output = Rule<Not<R>>;

    fn not(self) -> Self::Output { Rule(Not(self.0)) }
}
