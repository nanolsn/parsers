use super::prelude::*;

impl<'r, I, P0> Rule<'r, I> for (P0, )
    where
        P0: Rule<'r, I>,
{
    type Mat = P0::Mat;
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> { self.0.rule(input) }
}

impl<'r, I, P0, P1> Rule<'r, I> for (P0, P1)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
    }
}

impl<'r, I, P0, P1, P2> Rule<'r, I> for (P0, P1, P2)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
        P2: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat, P2::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
            .and_then(|(m0, m1), i| self.2.rule(i)
                .map(|m2| (m0, m1, m2))
            )
    }
}

impl<'r, I, P0, P1, P2, P3> Rule<'r, I> for (P0, P1, P2, P3)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
        P2: Rule<'r, I, Exp=P0::Exp>,
        P3: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
            .and_then(|(m0, m1), i| self.2.rule(i)
                .map(|m2| (m0, m1, m2))
            )
            .and_then(|(m0, m1, m2), i| self.3.rule(i)
                .map(|m3| (m0, m1, m2, m3))
            )
    }
}

impl<'r, I, P0, P1, P2, P3, P4> Rule<'r, I> for (P0, P1, P2, P3, P4)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
        P2: Rule<'r, I, Exp=P0::Exp>,
        P3: Rule<'r, I, Exp=P0::Exp>,
        P4: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
            .and_then(|(m0, m1), i| self.2.rule(i)
                .map(|m2| (m0, m1, m2))
            )
            .and_then(|(m0, m1, m2), i| self.3.rule(i)
                .map(|m3| (m0, m1, m2, m3))
            )
            .and_then(|(m0, m1, m2, m3), i| self.4.rule(i)
                .map(|m4| (m0, m1, m2, m3, m4))
            )
    }
}

impl<'r, I, P0, P1, P2, P3, P4, P5> Rule<'r, I> for (P0, P1, P2, P3, P4, P5)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
        P2: Rule<'r, I, Exp=P0::Exp>,
        P3: Rule<'r, I, Exp=P0::Exp>,
        P4: Rule<'r, I, Exp=P0::Exp>,
        P5: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat, P5::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
            .and_then(|(m0, m1), i| self.2.rule(i)
                .map(|m2| (m0, m1, m2))
            )
            .and_then(|(m0, m1, m2), i| self.3.rule(i)
                .map(|m3| (m0, m1, m2, m3))
            )
            .and_then(|(m0, m1, m2, m3), i| self.4.rule(i)
                .map(|m4| (m0, m1, m2, m3, m4))
            )
            .and_then(|(m0, m1, m2, m3, m4), i| self.5.rule(i)
                .map(|m5| (m0, m1, m2, m3, m4, m5))
            )
    }
}

impl<'r, I, P0, P1, P2, P3, P4, P5, P6> Rule<'r, I> for (P0, P1, P2, P3, P4, P5, P6)
    where
        P0: Rule<'r, I>,
        P1: Rule<'r, I, Exp=P0::Exp>,
        P2: Rule<'r, I, Exp=P0::Exp>,
        P3: Rule<'r, I, Exp=P0::Exp>,
        P4: Rule<'r, I, Exp=P0::Exp>,
        P5: Rule<'r, I, Exp=P0::Exp>,
        P6: Rule<'r, I, Exp=P0::Exp>,
{
    type Mat = (P0::Mat, P1::Mat, P2::Mat, P3::Mat, P4::Mat, P5::Mat, P6::Mat);
    type Exp = P0::Exp;

    fn rule(&'r self, input: I) -> Ruled<I, Self::Mat, Self::Exp> {
        self.0.rule(input)
            .and_then(|m0, i| self.1.rule(i)
                .map(|m1| (m0, m1))
            )
            .and_then(|(m0, m1), i| self.2.rule(i)
                .map(|m2| (m0, m1, m2))
            )
            .and_then(|(m0, m1, m2), i| self.3.rule(i)
                .map(|m3| (m0, m1, m2, m3))
            )
            .and_then(|(m0, m1, m2, m3), i| self.4.rule(i)
                .map(|m4| (m0, m1, m2, m3, m4))
            )
            .and_then(|(m0, m1, m2, m3, m4), i| self.5.rule(i)
                .map(|m5| (m0, m1, m2, m3, m4, m5))
            )
            .and_then(|(m0, m1, m2, m3, m4, m5), i| self.6.rule(i)
                .map(|m6| (m0, m1, m2, m3, m4, m5, m6))
            )
    }
}
