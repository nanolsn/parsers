use super::{
    apply::Apply,
    ruled::Ruled,
};

impl<I, P0> Apply<I> for (P0, )
    where
        P0: Apply<I>,
{
    type Err = P0::Err;
    type Res = P0::Res;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.0.apply(input) }
}

impl<I, P0, P1> Apply<I> for (P0, P1)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
    }
}

impl<I, P0, P1, P2> Apply<I> for (P0, P1, P2)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
        P2: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res, P2::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| self.2.apply(i)
                .map(|r2| (r0, r1, r2))
            )
    }
}

impl<I, P0, P1, P2, P3> Apply<I> for (P0, P1, P2, P3)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
        P2: Apply<I, Err=P0::Err>,
        P3: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| self.2.apply(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| self.3.apply(i)
                .map(|r3| (r0, r1, r2, r3))
            )
    }
}

impl<I, P0, P1, P2, P3, P4> Apply<I> for (P0, P1, P2, P3, P4)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
        P2: Apply<I, Err=P0::Err>,
        P3: Apply<I, Err=P0::Err>,
        P4: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res, P4::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| self.2.apply(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| self.3.apply(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| self.4.apply(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
    }
}

impl<I, P0, P1, P2, P3, P4, P5> Apply<I> for (P0, P1, P2, P3, P4, P5)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
        P2: Apply<I, Err=P0::Err>,
        P3: Apply<I, Err=P0::Err>,
        P4: Apply<I, Err=P0::Err>,
        P5: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res, P4::Res, P5::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| self.2.apply(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| self.3.apply(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| self.4.apply(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
            .and_then(|(r0, r1, r2, r3, r4), i| self.5.apply(i)
                .map(|r5| (r0, r1, r2, r3, r4, r5))
            )
    }
}

impl<I, P0, P1, P2, P3, P4, P5, P6> Apply<I> for (P0, P1, P2, P3, P4, P5, P6)
    where
        P0: Apply<I>,
        P1: Apply<I, Err=P0::Err>,
        P2: Apply<I, Err=P0::Err>,
        P3: Apply<I, Err=P0::Err>,
        P4: Apply<I, Err=P0::Err>,
        P5: Apply<I, Err=P0::Err>,
        P6: Apply<I, Err=P0::Err>,
{
    type Err = P0::Err;
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res, P4::Res, P5::Res, P6::Res);

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        self.0.apply(input)
            .and_then(|r0, i| self.1.apply(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| self.2.apply(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| self.3.apply(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| self.4.apply(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
            .and_then(|(r0, r1, r2, r3, r4), i| self.5.apply(i)
                .map(|r5| (r0, r1, r2, r3, r4, r5))
            )
            .and_then(|(r0, r1, r2, r3, r4, r5), i| self.6.apply(i)
                .map(|r6| (r0, r1, r2, r3, r4, r5, r6))
            )
    }
}
