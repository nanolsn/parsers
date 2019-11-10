mod xml;

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
use crate::{par, Parse, stringed_par, pred_fn};

#[test]
fn test() {
    let space = par(' ') * ..;
    let digit = pred_fn(pattern!('0'..='9'));
    let letter = pred_fn(pattern!('a'..='z')) | pred_fn(pattern!('A'..='Z'));
    let ident = stringed_par(letter | '_') & (letter | '_' | digit) * ..;
    let text = (letter | ' ') * (1..);
    let str_literal = par('\'') >> text << '\'';
    let num = digit * (1..);
    let float = num & '.' & par(move || digit) * ..;

    let to_str = str_literal.map(|s| Str(s));
    let to_ident = ident.map(|i| Ident(i));
    let to_float = float.map(|f: String| Float(f.as_str().parse().unwrap()));
    let to_num = num.map(|n: String| Number(n.as_str().parse().unwrap()));
    let float_or_num = to_float | to_num;

    let mul_rhs = space >> '*' >> space >> float_or_num;
    let mul_rhs = (mul_rhs ^ (1..)).reduce(|a, b| Mul(Box::new(a), Box::new(b)));

    let mul = par((float_or_num, mul_rhs));
    let to_mul = mul.map(|(l, r)| Mul(Box::new(l), Box::new(r)));

    let mul_or_num = to_mul | float_or_num;
    let sum = par((mul_or_num, space >> '+' >> space >> mul_or_num));
    let to_sum = sum.map(|(l, r)| Sum(Box::new(l), Box::new(r)));

    let to_var = to_sum.boxed()
        | to_mul.boxed()
        | to_float.boxed()
        | to_num.boxed()
        | to_str.boxed()
        | to_ident.boxed();

    let statement = space >> to_var << space << ';' << space;
    let to_statement = statement.map(|v| Statement(Box::new(v)));
    let code = to_statement ^ ..;

    assert_eq!(
        code.parse_result("_hello2;  12. *  42 *23 + 123*23 ;'text string'; "),
        Ok(vec![
            Statement(Box::new(
                Ident("_hello2".to_string()),
            )),
            Statement(Box::new(
                Sum(
                    Box::new(Mul(
                        Box::new(Float(12.0)),
                        Box::new(Mul(
                            Box::new(Number(42)),
                            Box::new(Number(23)),
                        )),
                    )),
                    Box::new(Mul(
                        Box::new(Number(123)),
                        Box::new(Number(23)),
                    )),
                ),
            )),
            Statement(Box::new(
                Str("text string".to_string()),
            )),
        ])
    );
}
