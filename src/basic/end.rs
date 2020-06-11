use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct End;

pub fn end() -> Rul<End> { Rul(End) }

impl<'i> Rule<&'i str> for End {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        if input.is_empty() {
            Ruled::Match("", "")
        } else {
            Ruled::Expected(SomeOf::AnyChar)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rul::rul;

    #[test]
    fn end() {
        let r = rul('a') << super::end();
        assert_eq!(r.rule("a"), Ruled::Match("a", ""));
        assert_eq!(r.rule("aa"), Ruled::Expected(SomeOf::AnyChar));
    }
}
