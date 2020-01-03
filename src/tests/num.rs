use crate::*;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Number {
    Bin(i64),
    Dec(i64),
    Hex(i64),
}

use Number::*;

/// Parses binary numbers like `0b0101`
fn bin_num<'o>() -> BoxedRule<'o, i64> {
    let zero = rule('0');
    let one = rule('1');
    let b = rule('b');
    let r = zero >> b >> (zero | one) * (1..);
    r
        .map(|s: String| i64::from_str_radix(s.as_str(), 2).unwrap())
        .boxed()
}

/// Parses decimal numbers like `296`
fn dec_num<'o>() -> BoxedRule<'o, i64> {
    let minus = rule('-');
    let r = minus & digit() * (1..);
    r
        .map(|s: String| i64::from_str(s.as_str()).unwrap())
        .boxed()
}

/// Parses hex numbers like `0x2B`
fn hex_num<'o>() -> BoxedRule<'o, i64> {
    let zero = rule('0');
    let x = rule('x');
    let r = zero >> x >> hex_digit() * (1..);
    r
        .map(|s: String| i64::from_str_radix(s.as_str(), 16).unwrap())
        .boxed()
}

/// Parses any number
fn any_num<'o>() -> BoxedRule<'o, Number> {
    let r =
        rule(bin_num).map(|n| Bin(n))
            | rule(hex_num).map(|n| Hex(n))
            | rule(dec_num).map(|n| Dec(n));

    r.boxed()
}

/// Parses multiple numbers and end of input
fn numbers<'o>() -> BoxedRule<'o, Vec<Number>> {
    let whites = white() * ..;
    let num = whites >> any_num;
    let r = (num ^ ..) << whites << end();
    r.boxed()
}

#[test]
fn numbers_parser() {
    let parser = Parser::new(" 0xFF -25 0b0101 ");

    assert_eq!(
        parser.parse_result(numbers),
        Ok(vec![Hex(255), Dec(-25), Bin(5)]),
    );
}
