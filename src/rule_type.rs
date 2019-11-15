use std::marker::PhantomData;
use crate::{Comply, Parser, Rule};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RuleType<R, E, P>(pub PhantomData<R>, pub PhantomData<E>, pub P);

impl<R, E, P> Copy for RuleType<R, E, P>
    where
        R: Clone,
        E: Clone,
        P: Copy,
{}

impl<'p, R, E, P> Comply<'p> for RuleType<R, E, P>
    where
        P: Comply<'p, Res=R, Err=E>,
        R: 'p,
        E: 'p,
{
    type Res = R;
    type Err = E;
    type On = P::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.2.comply(parser)
    }
}

impl<'p, R, E, P> From<Rule<P>> for RuleType<R, E, Rule<P>>
    where
        P: Comply<'p, Res=R, Err=E>,
        R: 'p,
        E: 'p,
{
    fn from(rule: Rule<P>) -> Self {
        RuleType(PhantomData, PhantomData, rule)
    }
}

impl<'p, R, E, P> Deref for RuleType<R, E, P>
    where
        P: Comply<'p, Res=R, Err=E>,
        R: 'p,
        E: 'p,
{
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn rule_type() {
        let r: RuleType<&str, (), _> = (rule('a') | 'b').into();
        let r: RuleType<String, (), _> = (rule(r) & '!').into();

        assert_eq!(
            Parser::new("a!").parse(r),
            (Ok("a!".to_string()), ""),
        );
        assert_eq!(
            Parser::new("b!").parse(r),
            (Ok("b!".to_string()), "")
        );
        assert_eq!(
            Parser::new("c!").parse(r),
            (Err(()), "c!")
        );
    }
}
