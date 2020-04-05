use super::{
    apply::Apply,
    ruled::Ruled,
    concat::Concat,
    rules::*,
};

/// The type wrapper to provide useful methods for a [rule].
///
/// [rule]: ./trait.Apply.html
#[derive(Copy, Clone, Debug)]
pub struct Rule<R>(pub R);

impl<R> Rule<R> {
    /// Applies [rules] then concatenate result.
    ///
    /// [rules]: ./trait.Apply.html
    pub fn cat<T, I, P>(self, rhs: P) -> Rule<Cat<T, R, P>>
        where
            R: Apply<I>,
            P: Apply<I, Err=R::Err>,
            T: Concat<R::Res, P::Res>,
    { Rule(Cat::new(self.0, rhs)) }

    /// Applies the first or second [rule].
    ///
    /// [rule]: ./trait.Apply.html
    pub fn or<I, P>(self, rhs: P) -> Rule<Or<R, P>>
        where
            R: Apply<I>,
            P: Apply<I, Err=R::Err>,
            I: Copy,
            R::Res: std::convert::Into<P::Res>,
    { Rule(Or(self.0, rhs)) }

    /// Applies the [rule] and reverse result.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn not<I>(self) -> Rule<Not<R>>
        where
            R: Apply<I>,
            I: Copy,
    { Rule(Not(self.0)) }

    /// Applies the [rule] then apply function `f` to the successful result.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn map<I, F, K>(self, f: F) -> Rule<Map<R, F>>
        where
            R: Apply<I>,
            F: FnOnce(R::Res) -> K,
    { Rule(Map(self.0, f)) }

    /// Applies the [rule] then apply function `f` to the error.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn map_err<I, F, Q>(self, f: F) -> Rule<MapErr<R, F>>
        where
            R: Apply<I>,
            F: FnOnce(R::Err) -> Q,
    { Rule(MapErr(self.0, f)) }

    /// Applies the [rule] and convert result into the type `T`.
    ///
    /// The result of the [rule] must implement the trait `std::convert::Into<T>`.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn into<T>(self) -> Rule<Into<T, R>>
    { Rule(Into::new(self.0)) }

    /// Applies the [rule] multiple times.
    ///
    /// The range specifies allowable number of applications.
    /// The results of applications will be concatenated into a value.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn range<T, I, B>(self, rng: B) -> Rule<Range<T, R>>
        where
            R: Apply<I>,
            I: Copy,
            T: Concat<T, R::Res>,
            B: std::ops::RangeBounds<usize>,
    { Rule(Range::from_range(self.0, rng)) }

    /// Applies the [rule] multiple times.
    ///
    /// The `times` value specifies the certain number of applications.
    /// The results of applications will be concatenated into a value.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn repeat<T, I>(self, times: usize) -> Rule<Range<T, R>>
        where
            R: Apply<I> + Copy,
            I: Copy,
            T: Concat<T, R::Res>,
    { Rule(Range::from_range(self.0, times..=times)) }

    /// Applies [rules] until a certain rule (`until`) is triggered.
    ///
    /// [rules]: ./trait.Apply.html
    pub fn until<T, I, U>(self, until: U) -> Rule<Until<T, R, U>>
        where
            R: Apply<I> + Copy,
            U: Apply<I> + Copy,
            I: Copy,
            T: Concat<T, R::Res>,
    { Rule(Until::new(self.0, until)) }

    /// Applies the [rule] and then call `f` to the result and applies it.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn and_then<I, F, K>(self, f: F) -> Rule<AndThen<R, F>>
        where
            R: Apply<I>,
            F: FnOnce(R::Res) -> K,
            K: Apply<I>,
            R::Err: std::convert::Into<K::Err>,
    { Rule(AndThen(self.0, f)) }

    /// Applies the [rule] or else call `f` to the error and applies it.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn or_else<I, F, K>(self, f: F) -> Rule<OrElse<R, F>>
        where
            R: Apply<I>,
            F: FnOnce(R::Err) -> K,
            K: Apply<I>,
            I: Copy,
            R::Res: std::convert::Into<K::Res>,
    { Rule(OrElse(self.0, f)) }

    /// Applies the predicate `p` to char and returns it if true
    pub fn pred<I, F>(self, p: F) -> Rule<Pred<R, F>>
        where
            R: Apply<I>,
            F: FnOnce(&R::Res) -> bool,
    { Rule(Pred(self.0, p)) }

    /// Try to apply the [rule] and returns `Option<Value>` of result.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn opt<I>(self) -> Rule<Opt<R>>
        where
            R: Apply<I>,
            I: Copy,
    { Rule(Opt(self.0)) }

    /// Try to apply the [rule] and returns default if fails.
    ///
    /// [rule]: ./trait.Apply.html
    pub fn or_default<I>(self) -> Rule<OrDefault<R>>
        where
            R: Apply<I>,
            I: Copy,
            R::Res: Default,
    { Rule(OrDefault(self.0)) }
}

impl<T, R, P> Rule<Cat<T, R, P>> {
    pub fn concat<I, K>(self, rhs: K) -> Rule<Cat<T, Cat<T, R, P>, K>>
        where
            R: Apply<I>,
            P: Apply<I, Err=R::Err>,
            K: Apply<I, Err=R::Err>,
            T: Concat<T, K::Res>,
    { Rule(Cat::new(self.0, rhs)) }
}

/// Constructor for [`Rule`].
///
/// [`Rule`]: ./struct.Rule.html
pub fn rule<R, I>(r: R) -> Rule<R>
    where
        R: Apply<I>,
{ Rule(r) }

impl<R, I> Apply<I> for Rule<R>
    where
        R: Apply<I>,
{
    type Err = R::Err;
    type Res = R::Res;

    fn apply(self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.0.apply(input) }
}

impl<R> std::ops::Deref for Rule<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<R> std::ops::DerefMut for Rule<R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<L, R> std::ops::BitAnd<R> for Rule<L> {
    type Output = Rule<Cat<String, L, R>>;

    fn bitand(self, rhs: R) -> Self::Output { Rule(Cat::new(self.0, rhs)) }
}

impl<L, R> std::ops::Add<R> for Rule<L> {
    type Output = Rule<Cat<&'static str, L, R>>;

    fn add(self, rhs: R) -> Self::Output { Rule(Cat::new(self.0, rhs)) }
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

impl<R, B> std::ops::Mul<B> for Rule<R>
    where
        B: std::ops::RangeBounds<usize>,
{
    type Output = Rule<Range<String, R>>;

    fn mul(self, rhs: B) -> Self::Output { Rule(Range::from_range(self.0, rhs)) }
}

impl<R> std::ops::Not for Rule<R> {
    type Output = Rule<Not<R>>;

    fn not(self) -> Self::Output { Rule(Not(self.0)) }
}
