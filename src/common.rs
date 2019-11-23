use crate::{Comply, Parser, Rule};

#[derive(Copy, Clone, Debug)]
pub struct Digit;

pub const fn digit() -> Rule<Digit> {
    Rule(Digit)
}

impl<'p> Comply<'p> for Digit {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c @ '0'..='9') => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct HexDigit;

pub const fn hex_digit() -> Rule<HexDigit> {
    Rule(HexDigit)
}

impl<'p> Comply<'p> for HexDigit {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c @ '0'..='9') => Ok(parser.step(c.len_utf8())),
            Some(c @ 'a'..='f') => Ok(parser.step(c.len_utf8())),
            Some(c @ 'A'..='F') => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Space;

pub const fn space() -> Rule<Space> {
    Rule(Space)
}

impl<'p> Comply<'p> for Space {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(' ') => Ok(parser.step(' '.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct White;

pub const fn white() -> Rule<White> {
    Rule(White)
}

impl<'p> Comply<'p> for White {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let nl = "\r\n";
        if parser.rest().starts_with(nl) {
            return Ok(parser.step(nl.len()))
        }

        match parser.rest().chars().next() {
            Some(c @ ' ') => Ok(parser.step(c.len_utf8())),
            Some(c @ '\n') => Ok(parser.step(c.len_utf8())),
            Some(c @ '\r') => Ok(parser.step(c.len_utf8())),
            Some(c @ '\t') => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NewLine;

pub const fn new_line() -> Rule<NewLine> {
    Rule(NewLine)
}

impl<'p> Comply<'p> for NewLine {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let nl = "\r\n";
        if parser.rest().starts_with(nl) {
            return Ok(parser.step(nl.len()))
        }

        match parser.rest().chars().next() {
            Some(c @ '\n') => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Alpha;

pub const fn alpha() -> Rule<Alpha> {
    Rule(Alpha)
}

impl<'p> Comply<'p> for Alpha {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c @ 'a'..='z') => Ok(parser.step(c.len_utf8())),
            Some(c @ 'A'..='Z') => Ok(parser.step(c.len_utf8())),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Any;

impl<'p> Comply<'p> for Any {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            None => Err(()),
            Some(c) => Ok(parser.step(c.len_utf8())),
        }
    }
}

pub const fn any() -> Rule<Any> {
    Rule(Any)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit() {
        assert_eq!(
            Parser::new("0").parse(super::digit()),
            (Ok("0"), ""),
        );
        assert_eq!(
            Parser::new("9").parse(super::digit()),
            (Ok("9"), ""),
        );
        assert_eq!(
            Parser::new("a").parse(super::digit()),
            (Err(()), "a"),
        );
    }

    #[test]
    fn hex_digit() {
        assert_eq!(
            Parser::new("0").parse(super::hex_digit()),
            (Ok("0"), ""),
        );
        assert_eq!(
            Parser::new("a").parse(super::hex_digit()),
            (Ok("a"), ""),
        );
        assert_eq!(
            Parser::new("g").parse(super::hex_digit()),
            (Err(()), "g"),
        );
    }

    #[test]
    fn space() {
        assert_eq!(
            Parser::new(" ").parse(super::space()),
            (Ok(" "), ""),
        );
        assert_eq!(
            Parser::new("a").parse(super::space()),
            (Err(()), "a"),
        );
    }

    #[test]
    fn white() {
        assert_eq!(
            Parser::new("\r\n ").parse(super::white()),
            (Ok("\r\n"), " "),
        );
        assert_eq!(
            Parser::new("\n ").parse(super::white()),
            (Ok("\n"), " "),
        );
        assert_eq!(
            Parser::new("\r ").parse(super::white()),
            (Ok("\r"), " "),
        );
        assert_eq!(
            Parser::new("\t ").parse(super::white()),
            (Ok("\t"), " "),
        );
        assert_eq!(
            Parser::new("  ").parse(super::white()),
            (Ok(" "), " "),
        );
        assert_eq!(
            Parser::new("a ").parse(super::white()),
            (Err(()), "a "),
        );
    }

    #[test]
    fn new_line() {
        assert_eq!(
            Parser::new("\r\n").parse(super::new_line()),
            (Ok("\r\n"), ""),
        );
        assert_eq!(
            Parser::new("\n").parse(super::new_line()),
            (Ok("\n"), ""),
        );
        assert_eq!(
            Parser::new("a").parse(super::new_line()),
            (Err(()), "a"),
        );
    }

    #[test]
    fn alpha() {
        assert_eq!(
            Parser::new("a").parse(super::alpha()),
            (Ok("a"), ""),
        );
        assert_eq!(
            Parser::new("Z").parse(super::alpha()),
            (Ok("Z"), ""),
        );
        assert_eq!(
            Parser::new("1").parse(super::alpha()),
            (Err(()), "1"),
        );
        assert_eq!(
            Parser::new("").parse(super::alpha()),
            (Err(()), ""),
        );
    }

    #[test]
    fn any() {
        assert_eq!(
            Parser::new("%^&").parse(super::any()),
            (Ok("%"), "^&"),
        );
        assert_eq!(
            Parser::new("").parse(super::any()),
            (Err(()), "")
        );
    }
}
