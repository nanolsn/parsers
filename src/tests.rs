#![cfg(test)]

#[derive(Debug, PartialOrd, PartialEq)]
enum Var {
    Number(u32),
    Float(f32),
    Str(String),
    Ident(String),
    Sum(Box<Var>, Box<Var>),
    Mul(Box<Var>, Box<Var>),
    Statement(Box<Var>),
}

use Var::*;
use crate::{par, Parse, stringed_par};

#[test]
fn test() {
    let space = par(' ') * ..;
    let digit = par(pattern!('0'..='9'));
    let letter = par(pattern!('a'..='z')) | pattern!('A'..='Z');
    let letter_or_underscore = letter | '_';
    let ident = stringed_par(letter_or_underscore) & (letter_or_underscore | digit) * ..;
    let text = (letter | ' ') * (1..);
    let str_literal = par('\'') >> (text << '\'');
    let num = digit * (1..);
    let float = num & '.' & (digit * ..);

    let to_str = str_literal.map(|s| Str(s));
    let to_ident = ident.map(|i| Ident(i));
    let to_float = float.map(|f: String| Float(f.as_str().parse().unwrap()));
    let to_num = num.map(|n: String| Number(n.as_str().parse().unwrap()));
    let float_or_num = to_float | to_num;

    let mul = par((float_or_num, space >> '*' >> space >> float_or_num));
    let to_mul = mul.map(|(l, r)| Mul(Box::new(l), Box::new(r)));

    let mul_or_num = to_mul | float_or_num;
    let sum = par((mul_or_num, space >> '+' >> space >> mul_or_num));
    let to_sum = sum.map(|(l, r)| Sum(Box::new(l), Box::new(r)));

    let to_var = to_sum | to_mul | to_float | to_num | to_str | to_ident;

    let statement = space >> (to_var << space << ';' << space);
    let to_statement = statement.map(|v| Statement(Box::new(v)));
    let code = to_statement ^ ..;

    assert_eq!(
        code.parse_result("_hello2;  12. *  42 + 123*23 ;'text string'; "),
        Ok(vec![
            Statement(Box::new(
                Ident("_hello2".to_string()),
            )),
            Statement(Box::new(
                Sum(
                    Box::new(Mul(
                        Box::new(Float(12.0)),
                        Box::new(Number(42))),
                    ),
                    Box::new(Mul(
                        Box::new(Number(123)),
                        Box::new(Number(23))
                    )),
                ),
            )),
            Statement(Box::new(
                Str("text string".to_string()),
            )),
        ])
    );
}
