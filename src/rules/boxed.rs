use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

pub type BoxedRule<I, R, E = ()> = Rule<Box<dyn Apply<I, Res=R, Err=E>>>;

impl<I, R, E> Apply<I> for Box<dyn Apply<I, Res=R, Err=E>> {
    type Err = E;
    type Res = R;

    fn apply(&self, input: I) -> Ruled<I, Self::Res, Self::Err> { self.as_ref().apply(input) }
}

pub fn boxed<I, R>(rule: R) -> BoxedRule<I, R::Res, R::Err>
    where
        R: Apply<I> + 'static,
{ Rule(Box::new(rule)) }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        apply::apply,
        rule::rule,
    };

    #[test]
    fn or() {
        fn get_rule() -> BoxedRule<&'static str, String> {
            boxed(rule("@") & '#')
        }

        let r = get_rule();
        assert_eq!(apply(r, "@#"), Ruled::Ok("@#".to_owned(), ""));
    }
}
