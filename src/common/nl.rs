use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct Nl;

pub fn nl() -> Rule<Nl> { Rule(Nl) }

impl<'i> Apply<&'i str> for Nl {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        match input.chars().next() {
            Some(c @ '\n') => input.split_at(c.len_utf8()).into(),
            Some(c @ '\r') => input.split_at(c.len_utf8()).into(),
            _ => return Ruled::Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::apply::apply;

    #[test]
    fn nl() {
        assert!(apply(&super::nl(), "\n").is_ok());
        assert!(apply(&super::nl(), "\r").is_ok());
        assert!(apply(&super::nl(), "\r\n").is_ok());
        assert!(apply(&super::nl(), "~").is_err());
    }
}
