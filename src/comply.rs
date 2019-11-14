use crate::Parser;

pub trait Comply<'p> {
    type Res: 'p;
    type Err: 'p;
    type On: 'p;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err>;
}

impl<'p> Comply<'p> for char {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        match parser.rest().chars().next() {
            Some(c) if c == *self => {
                Ok(parser.step(c.len_utf8()))
            }
            _ => Err(())
        }
    }
}

impl<'p> Comply<'p> for str {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        if parser.rest().starts_with(self) {
            Ok(parser.step(self.len()))
        } else {
            Err(())
        }
    }
}

impl<'p> Comply<'p> for String {
    type Res = &'p str;
    type Err = ();
    type On = &'p str;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.as_str().comply(parser)
    }
}

impl<'p, T> Comply<'p> for &T
    where
        T: Comply<'p> + ?Sized,
{
    type Res = T::Res;
    type Err = T::Err;
    type On = T::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        (*self).comply(parser)
    }
}

impl<'p> Comply<'p> for () {
    type Res = String;
    type Err = ();
    type On = &'p str;

    fn comply(&self, _: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        Ok(String::new())
    }
}

impl<'p, P0> Comply<'p> for (P0, )
    where
        P0: Comply<'p>,
{
    type Res = P0::Res;
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        self.0.comply(parser)
    }
}

impl<'p, P0, P1> Comply<'p> for (P0, P1)
    where
        P0: Comply<'p>,
        P1: Comply<'p, Err=P0::Err, On=P0::On>,
{
    type Res = (P0::Res, P1::Res);
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let p0 = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let p1 = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok((p0, p1))
    }
}

impl<'p, P0, P1, P2> Comply<'p> for (P0, P1, P2)
    where
        P0: Comply<'p>,
        P1: Comply<'p, Err=P0::Err, On=P0::On>,
        P2: Comply<'p, Err=P0::Err, On=P0::On>,
{
    type Res = (P0::Res, P1::Res, P2::Res);
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let p0 = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let p1 = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p2 = self.2.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok((p0, p1, p2))
    }
}

impl<'p, P0, P1, P2, P3> Comply<'p> for (P0, P1, P2, P3)
    where
        P0: Comply<'p>,
        P1: Comply<'p, Err=P0::Err, On=P0::On>,
        P2: Comply<'p, Err=P0::Err, On=P0::On>,
        P3: Comply<'p, Err=P0::Err, On=P0::On>,
{
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res);
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let p0 = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let p1 = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p2 = self.2.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p3 = self.3.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok((p0, p1, p2, p3))
    }
}

impl<'p, P0, P1, P2, P3, P4> Comply<'p> for (P0, P1, P2, P3, P4)
    where
        P0: Comply<'p>,
        P1: Comply<'p, Err=P0::Err, On=P0::On>,
        P2: Comply<'p, Err=P0::Err, On=P0::On>,
        P3: Comply<'p, Err=P0::Err, On=P0::On>,
        P4: Comply<'p, Err=P0::Err, On=P0::On>,
{
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res, P4::Res);
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let p0 = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let p1 = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p2 = self.2.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p3 = self.3.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p4 = self.4.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok((p0, p1, p2, p3, p4))
    }
}

impl<'p, P0, P1, P2, P3, P4, P5> Comply<'p> for (P0, P1, P2, P3, P4, P5)
    where
        P0: Comply<'p>,
        P1: Comply<'p, Err=P0::Err, On=P0::On>,
        P2: Comply<'p, Err=P0::Err, On=P0::On>,
        P3: Comply<'p, Err=P0::Err, On=P0::On>,
        P4: Comply<'p, Err=P0::Err, On=P0::On>,
        P5: Comply<'p, Err=P0::Err, On=P0::On>,
{
    type Res = (P0::Res, P1::Res, P2::Res, P3::Res, P4::Res, P5::Res);
    type Err = P0::Err;
    type On = P0::On;

    fn comply(&self, parser: &mut Parser<Self::On>) -> Result<Self::Res, Self::Err> {
        let pos = parser.get_pos();
        let p0 = self.0.comply(parser).map_err(|e| {
            assert_eq!(parser.get_pos(), pos);
            e
        })?;

        let p1 = self.1.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p2 = self.2.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p3 = self.3.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p4 = self.4.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        let p5 = self.5.comply(parser).map_err(|e| {
            parser.set_pos(pos);
            e
        })?;

        Ok((p0, p1, p2, p3, p4, p5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule;

    #[test]
    fn comply_str() {
        let hello = "hello".to_string();
        let r = rule(hello);

        assert_eq!(
            Parser::new("hello.").parse(r.clone()),
            (Ok("hello"), "."),
        );
        assert_eq!(
            Parser::new("hi").parse(r),
            (Err(()), "hi"),
        );
    }

    #[test]
    fn comply_char() {
        let r = rule('@');

        assert_eq!(
            Parser::new("@#".to_owned().as_str()).parse(r),
            (Ok("@"), "#"),
        );
        assert_eq!(
            Parser::new("$").parse(r),
            (Err(()), "$"),
        );
    }

    #[test]
    fn comply_tuple() {
        let r = (rule('@'), '#', "__");

        assert_eq!(
            Parser::new("@#__".to_owned().as_str()).parse(r),
            (Ok(("@", "#", "__")), ""),
        );
        assert_eq!(
            Parser::new("@#!").parse(r),
            (Err(()), "@#!"),
        );
        assert_eq!(
            Parser::new("#$").parse(r),
            (Err(()), "#$"),
        );

        let r = (rule('0'), '1', "23", "4");

        assert_eq!(
            Parser::new("012345".to_owned().as_str()).parse(r),
            (Ok(("0", "1", "23", "4")), "5"),
        );
        assert_eq!(
            Parser::new("0123").parse(r),
            (Err(()), "0123"),
        );
        assert_eq!(
            Parser::new("0.").parse(r),
            (Err(()), "0."),
        );
    }

    #[test]
    fn comply_tuple_6() {
        let r = (rule('0'), '1', '2', '3', '4', '5');

        assert_eq!(
            Parser::new("0123456").parse(r),
            (Ok(("0", "1", "2", "3", "4", "5")), "6"),
        );
        assert_eq!(
            Parser::new("012_456").parse(r),
            (Err(()), "012_456"),
        );
    }
}
