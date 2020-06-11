use crate::{
    rule::Rule,
    ruled::Ruled,
    rul::Rul,
    some_of::SomeOf,
};

#[derive(Copy, Clone, Debug)]
pub struct Nl;

pub fn nl() -> Rul<Nl> { Rul(Nl) }

impl<'i> Rule<&'i str> for Nl {
    type Exp = SomeOf<'static>;
    type Mat = &'i str;

    fn rule(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        match input.chars().next() {
            Some(c @ '\n') => input.split_at(c.len_utf8()).into(),
            Some(c @ '\r') => input.split_at(c.len_utf8()).into(),
            _ => return Ruled::Expected(SomeOf::Nl),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nl() {
        assert!(super::nl().rule("\n").is_match());
        assert!(super::nl().rule("\r").is_match());
        assert!(super::nl().rule("\r\n").is_match());
        assert!(super::nl().rule("~").is_expected());
    }
}
