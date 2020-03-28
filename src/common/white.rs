use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::Rule,
};

#[derive(Copy, Clone, Debug)]
pub struct White;

pub fn white() -> Rule<White> { Rule(White) }

impl<'i> Apply<&'i str> for White {
    type Err = ();
    type Res = &'i str;

    fn apply(&self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let nl = "\r\n";
        if input.starts_with(nl) {
            return input.split_at(nl.len()).into();
        }

        let c = match input.chars().next() {
            Some(c @ ' ') => c,
            Some(c @ '\n') => c,
            Some(c @ '\r') => c,
            Some(c @ '\t') => c,
            _ => return Ruled::Err(()),
        };

        input.split_at(c.len_utf8()).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::apply::apply;

    #[test]
    fn white() {
        assert!(apply(&super::white(), " ").is_ok());
        assert!(apply(&super::white(), "\n").is_ok());
        assert!(apply(&super::white(), "\t").is_ok());
        assert!(apply(&super::white(), "!").is_err());
    }
}
