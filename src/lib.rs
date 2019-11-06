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
    pub mod repeat_concat;
    pub mod range;
    pub mod range_concat;
    pub mod concat;
    pub mod any;
    pub mod until;
    pub mod until_concat;
}

pub use parsers::{
    parser::{par, Parser},
    list_parser::{ListParser, HeadParser},
    second::Second,
    first::First,
    or_parser::OrParser,
    repeat::Repeat,
    repeat_concat::RepeatConcat,
    range::Range,
    range_concat::RangeConcat,
    concat::Concat,
    any::{Any, ANY},
    until::Until,
    until_concat::UntilConcat,
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

    #[test]
    fn test() {
        let digit = par(match_this!('0'..='9'));
        let space = par(' ') ^ ..;
        let what = space >> (digit ^ (1..));

        assert_eq!(what.parse("    12@"), Ok(("12", "@")));

        //let float = (what & '.' & (digit ^ ..));
            //.map(|s: &str| s.parse::<f32>().unwrap());

        //assert_eq!(float.parse_result("  12.45"), Ok("12.45"));
    }
}
