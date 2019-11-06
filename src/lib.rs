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
};

pub use parse::Parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = par("hello") & ' ' & "world" & '!';

        assert_eq!(p.parse("hello world!"), Ok(("hello world!", "")));
    }
}
