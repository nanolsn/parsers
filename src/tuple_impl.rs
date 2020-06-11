use super::{
    rule::Rule,
    ruled::Ruled,
};

impl<I, P0> Rule<I> for (P0, )
    where
        P0: Rule<I>,
{
    type Exp = P0::Exp;
    type Mat = P0::Mat;

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.0.rule(input) }
}

impl<I, P0, P1> Rule<I> for (P0, P1)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
    }
}

impl<I, P0, P1, P2> Rule<I> for (P0, P1, P2)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
        P2: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat, P2::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1, p2) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| p2.rule(i)
                .map(|r2| (r0, r1, r2))
            )
    }
}

impl<I, P0, P1, P2, P3> Rule<I> for (P0, P1, P2, P3)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
        P2: Rule<I, Exp=P0::Exp>,
        P3: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1, p2, p3) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| p2.rule(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| p3.rule(i)
                .map(|r3| (r0, r1, r2, r3))
            )
    }
}

impl<I, P0, P1, P2, P3, P4> Rule<I> for (P0, P1, P2, P3, P4)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
        P2: Rule<I, Exp=P0::Exp>,
        P3: Rule<I, Exp=P0::Exp>,
        P4: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1, p2, p3, p4) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| p2.rule(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| p3.rule(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| p4.rule(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
    }
}

impl<I, P0, P1, P2, P3, P4, P5> Rule<I> for (P0, P1, P2, P3, P4, P5)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
        P2: Rule<I, Exp=P0::Exp>,
        P3: Rule<I, Exp=P0::Exp>,
        P4: Rule<I, Exp=P0::Exp>,
        P5: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat, P5::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1, p2, p3, p4, p5) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| p2.rule(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| p3.rule(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| p4.rule(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
            .and_then(|(r0, r1, r2, r3, r4), i| p5.rule(i)
                .map(|r5| (r0, r1, r2, r3, r4, r5))
            )
    }
}

impl<I, P0, P1, P2, P3, P4, P5, P6> Rule<I> for (P0, P1, P2, P3, P4, P5, P6)
    where
        P0: Rule<I>,
        P1: Rule<I, Exp=P0::Exp>,
        P2: Rule<I, Exp=P0::Exp>,
        P3: Rule<I, Exp=P0::Exp>,
        P4: Rule<I, Exp=P0::Exp>,
        P5: Rule<I, Exp=P0::Exp>,
        P6: Rule<I, Exp=P0::Exp>,
{
    type Exp = P0::Exp;
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat, P5::Mat, P6::Mat);

    fn rule(self, input: I) -> Ruled<I, Self::Res, Self::Err> {
        let (p0, p1, p2, p3, p4, p5, p6) = self;

        p0.rule(input)
            .and_then(|r0, i| p1.rule(i)
                .map(|r1| (r0, r1))
            )
            .and_then(|(r0, r1), i| p2.rule(i)
                .map(|r2| (r0, r1, r2))
            )
            .and_then(|(r0, r1, r2), i| p3.rule(i)
                .map(|r3| (r0, r1, r2, r3))
            )
            .and_then(|(r0, r1, r2, r3), i| p4.rule(i)
                .map(|r4| (r0, r1, r2, r3, r4))
            )
            .and_then(|(r0, r1, r2, r3, r4), i| p5.rule(i)
                .map(|r5| (r0, r1, r2, r3, r4, r5))
            )
            .and_then(|(r0, r1, r2, r3, r4, r5), i| p6.rule(i)
                .map(|r6| (r0, r1, r2, r3, r4, r5, r6))
            )
    }
}
