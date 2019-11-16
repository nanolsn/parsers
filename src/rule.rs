use crate::{
    Comply,
    Until,
    UntilVec,
    AndThen,
    OrElse,
    Map,
    MapErr,
    BoxedRule,
    Opt,
    Pred,
    Range,
    RangeVec,
    Or,
    Concat,
    Not,
};
use crate::Parser;

#[derive(Copy, Clone, Debug)]
pub struct Rule<R>(pub R);

pub fn rule<'p, R>(rule: R) -> Rule<R>
    where
        R: Comply<'p>,
{
    Rule(rule)
}

impl<R> Rule<R> {
    pub fn and<'p, P>(self, rule: P) -> Rule<Concat<R, P>>
        where
            R: Comply<'p>,
            P: Comply<'p, Err=R::Err, On=R::On>,
    {
        Rule(Concat(self.0, rule))
    }

    pub fn or<'p, P>(self, rule: P) -> Rule<Or<R, P>>
        where
            R: Comply<'p>,
            P: Comply<'p, Res=R::Res, Err=R::Err, On=R::On>,
    {
        Rule(Or(self.0, rule))
    }

    pub fn not<'p>(self) -> Rule<Not<R>>
        where
            R: Comply<'p>,
    {
        Rule(Not(self.0))
    }

    pub fn and_then<'p, F, N>(self, f: F) -> Rule<AndThen<R, F>>
        where
            R: Comply<'p>,
            F: Fn(R::Res) -> N,
            N: Comply<'p, Err=R::Err, On=R::On> + 'p,
    {
        Rule(AndThen(self.0, f))
    }

    pub fn or_else<'p, F, N>(self, f: F) -> Rule<OrElse<R, F>>
        where
            R: Comply<'p>,
            F: Fn(R::Err) -> N,
            N: Comply<'p, Res=R::Res, On=R::On>,
    {
        Rule(OrElse(self.0, f))
    }

    pub fn map<'p, F, K>(self, f: F) -> Rule<Map<R, F>>
        where
            R: Comply<'p>,
            F: Fn(R::Res) -> K,
            K: 'p,
    {
        Rule(Map(self.0, f))
    }

    pub fn map_err<'p, F, G>(self, f: F) -> Rule<MapErr<R, F>>
        where
            R: Comply<'p>,
            F: Fn(R::Err) -> G,
            G: 'p,
    {
        Rule(MapErr(self.0, f))
    }

    pub fn until<'p, U, S>(self, until: U) -> Rule<Until<R, U>>
        where
            R: Comply<'p, Res=S, On=&'p str>,
            S: AsRef<str> + 'p,
            U: Comply<'p, On=&'p str>,
    {
        Rule(Until(self.0, until))
    }

    pub fn until_vec<'p, U>(self, until: U) -> Rule<UntilVec<R, U>>
        where
            R: Comply<'p>,
            U: Comply<'p, On=R::On>,
            R::On: Copy,
    {
        Rule(UntilVec(self.0, until))
    }

    pub fn pred<'p, F>(self, f: F) -> Rule<Pred<R, F>>
        where
            R: Comply<'p>,
            F: Fn(&R::Res) -> bool,
            R::Res: 'p,
    {
        Rule(Pred(self.0, f))
    }

    pub fn opt<'p>(self) -> Rule<Opt<R>>
        where
            R: Comply<'p, On=&'p str>,
    {
        Rule(Opt(self.0))
    }

    pub fn boxed<'p>(self) -> BoxedRule<'p, R::Res, R::Err, R::On>
        where
            R: Comply<'p> + 'p,
    {
        Rule(Box::new(self.0))
    }

    pub fn repeat<'p, S>(self, times: usize) -> Rule<Range<R>>
        where
            R: Comply<'p, Res=S>,
            S: AsRef<str> + 'p,
            R::On: Copy,
    {
        Rule(Range {
            rule: self.0,
            from: times,
            to: Some(times),
        })
    }

    pub fn range<'p, S>(self, from: usize, to: usize) -> Rule<Range<R>>
        where
            R: Comply<'p, Res=S>,
            S: AsRef<str> + 'p,
            R::On: Copy,
    {
        Rule(Range {
            rule: self.0,
            from,
            to: Some(to),
        })
    }

    pub fn n_or_more<'p, S>(self, n: usize) -> Rule<Range<R>>
        where
            R: Comply<'p, Res=S>,
            S: AsRef<str> + 'p,
            R::On: Copy,
    {
        Rule(Range {
            rule: self.0,
            from: n,
            to: None,
        })
    }

    pub fn range_vec<'p, S>(self, from: usize, to: usize) -> Rule<RangeVec<R>>
        where
            R: Comply<'p, Res=S>,
            S: AsRef<str> + 'p,
            R::On: Copy,
    {
        Rule(RangeVec {
            rule: self.0,
            from,
            to: Some(to),
        })
    }

    pub fn n_or_more_vec<'p, S>(self, n: usize) -> Rule<RangeVec<R>>
        where
            R: Comply<'p, Res=S>,
            S: AsRef<str> + 'p,
            R::On: Copy,
    {
        Rule(RangeVec {
            rule: self.0,
            from: n,
            to: None,
        })
    }
}

impl<'p, R> Comply<'p> for Rule<R>
    where
        R: Comply<'p>,
{
    type Res = R::Res;
    type Err = R::Err;
    type On = R::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser)
    }
}
