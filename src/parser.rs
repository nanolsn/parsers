use crate::Comply;

#[derive(Copy, Clone, Debug)]
pub struct ParseInfo<I, R, E> {
    pub status: Result<R, E>,
    pub input: I,
    pub rest: I,
    pub pos: usize,
}

#[derive(Debug)]
pub struct Parser<I> {
    input: I,
    pos: usize,
}

impl<I> Parser<I> {
    pub fn new(input: I) -> Self {
        Parser {
            input,
            pos: 0,
        }
    }

    pub fn get_input(&self) -> &I {
        &self.input
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn parse_result<'p, R>(mut self, rule: R) -> Result<R::Res, R::Err>
        where
            R: Comply<'p, On=I>,
            I: 'p,
    {
        rule.comply(&mut self)
    }

    pub fn parse_ok<'p, R>(mut self, rule: R) -> Option<R::Res>
        where
            R: Comply<'p, On=I>,
            I: 'p,
    {
        rule.comply(&mut self).ok()
    }

    pub fn parse_err<'p, R>(mut self, rule: R) -> Option<R::Err>
        where
            R: Comply<'p, On=I>,
            I: 'p,
    {
        rule.comply(&mut self).err()
    }
}

impl<'o> Parser<&'o str> {
    pub fn step(&mut self, n: usize) -> &'o str {
        let pos = self.pos;
        self.pos += n;
        &self.input[pos..self.pos]
    }

    pub fn rest(&self) -> &'o str {
        &self.input[self.pos..]
    }

    pub fn parse<'p, R>(mut self, rule: R) -> (Result<R::Res, R::Err>, &'o str)
        where
            R: Comply<'p, On=&'o str>,
    {
        (rule.comply(&mut self), self.rest())
    }

    pub fn parse_info<'p, R>(mut self, rule: R) -> ParseInfo<&'o str, R::Res, R::Err>
        where
            R: Comply<'p, On=&'o str>,
    {
        ParseInfo {
            status: rule.comply(&mut self),
            input: self.input,
            rest: self.rest(),
            pos: self.pos,
        }
    }
}
