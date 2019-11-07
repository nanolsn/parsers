#![cfg(test)]

#[derive(Debug, PartialOrd, PartialEq)]
enum Var {
    Number(u32),
    Float(f32),
    Word(String),
}

use Var::*;
use crate::{par, Parse};

#[test]
fn test() {
    let space = par(' ') * ..;
    let word = (par(pattern!('a'..='z')) | pattern!('A'..='Z')) * (1..);
    let digit = par(pattern!('0'..='9'));
    let num = digit * (1..);
    let float = num & '.' & (digit * ..);

    let var = float.map(|f: String| Float(f.as_str().parse().unwrap()))
        | num.map(|n: String| Number(n.as_str().parse().unwrap()))
        | word.map(|w| Word(w));

    let code = (space >> var) ^ ..;

    assert_eq!(
        code.parse_result("  12. 42qwe"),
        Ok(vec![
            Float(12.0),
            Number(42),
            Word("qwe".to_string()),
        ])
    );
}
