use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::{Rule, rule},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Gender {
    Boy,
    Girl,
    Octopus,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Boy => "boy",
            Gender::Girl => "girl",
            Gender::Octopus => "octopus",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoodBoy {
    user: String,
    gender: Gender,
}

impl GoodBoy {
    pub fn new<S>(user: S, gender: Gender) -> Self
        where
            S: Into<String>,
    { GoodBoy { user: user.into(), gender } }
}

pub fn good_boy<S>(user: S, gender: Gender) -> Rule<GoodBoy>
    where
        S: Into<String>,
{ Rule(GoodBoy::new(user, gender)) }

impl<'i> Apply<&'i str> for GoodBoy {
    type Err = ();
    type Res = String;

    fn apply(self, input: &'i str) -> Ruled<&'i str, Self::Res, Self::Err> {
        let good = rule(self.user.as_str())
            & " is a good "
            & self.gender.as_str()
            & '!';

        good.apply(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apply::apply;

    #[test]
    fn good_boy() {
        let boy = "Nano";

        let result = apply(super::good_boy(boy, Gender::Boy), "Nano is a good boy!");
        assert!(result.is_ok());

        let result = apply(super::good_boy(boy, Gender::Boy), "Nano is NOT a good boy!");
        assert!(result.is_err());
    }
}
