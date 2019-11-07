#[macro_use]
mod impl_tuple;
mod parse;
mod maps;

mod parsers {
    pub mod parser;
    pub mod list_parser;
    pub mod second;
    pub mod first;
    pub mod or_parser;
    pub mod repeat;
    pub mod range;
    pub mod range_vec;
    pub mod concat;
    pub mod any;
    pub mod until;
}

pub use parsers::{
    parser::{par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    range::Range,
    concat::Concat,
    any::{Any, ANY},
    until::Until,
    range_vec::RangeVec,
};

pub use parse::Parse;

#[macro_export]
macro_rules! match_this {
    ($p:pat) => {
        |a| match a {
            $p => true,
            _ => false,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialOrd, PartialEq)]
    enum Var {
        Number(u32),
        Float(f32),
        Word(String),
    }

    #[test]
    fn test() {
        let space = par(' ') * ..;
        let word = (par(match_this!('a'..='z')) | match_this!('A'..='Z')) * (1..);
        let digit = par(match_this!('0'..='9'));
        let num = digit * (1..);
        let float = num & '.' & (digit * ..);

        let var = space >>
            (float.map(|f: String| Var::Float(f.as_str().parse::<f32>().unwrap()))
                | num.map(|n: String| Var::Number(n.as_str().parse::<u32>().unwrap()))
                | word.map(|w| Var::Word(w)));

        let code = var ^ ..;

        assert_eq!(
            code.parse_result("12.45 42  qwe"),
            Ok(vec![
                Var::Float(12.45),
                Var::Number(42),
                Var::Word("qwe".to_string()),
            ])
        );
    }
}
