use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

pub type BoxedRule<I, R, E = ()> = Rule<Box<dyn Apply<I, Res=R, Err=E>>>;

pub fn boxed<I, R>(rule: R) -> BoxedRule<I, R::Res, R::Err>
    where
        R: Apply<I> + 'static,
{ Rule(Box::new(rule)) }

impl<I, R, E> Apply<I> for Box<dyn Apply<I, Res=R, Err=E>> {
    type Err = E;
    type Res = R;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.as_ref().apply(input) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    #[test]
    fn boxed() {
        fn get_rule() -> BoxedRule<&'static str, String> { super::boxed(rule("@") & '#') }

        let r = get_rule();
        assert_eq!(apply(&r, "@#"), Ruled::Ok("@#".to_owned(), ""));
    }
}
