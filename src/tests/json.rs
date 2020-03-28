use std::{
    collections::HashMap,
    str::FromStr,
};

use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::rule,
    common::{
        dec::dec,
        bin::bin,
        oct::oct,
        hex::hex,
    },
};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum Json<'i> {
    Num(i64),
    Str(&'i str),
    Bool(bool),
    Array(Vec<Json<'i>>),
    Obj(HashMap<&'i str, Json<'i>>),
}

#[derive(Debug)]
enum JsonError {
    UndefinedError,
    IncorrectNum(String),
}

fn read_num(input: &str) -> Ruled<&str, Json, JsonError> {
    let minus = rule('-').or_default();
    let dec = minus & dec() * (1..);

    let bin = rule("0b") >> bin() * (1..);
    let oct = rule("0o") >> oct() * (1..);
    let hex = rule("0x") >> hex() * (1..);

    let num =
        bin.map(|s| i64::from_str_radix(&*s, 2).map_err(|_| JsonError::IncorrectNum(s)))
            | oct.map(|s| i64::from_str_radix(&*s, 8).map_err(|_| JsonError::IncorrectNum(s)))
            | hex.map(|s| i64::from_str_radix(&*s, 16).map_err(|_| JsonError::IncorrectNum(s)))
            | dec.map(|s| i64::from_str(&*s).map_err(|_| JsonError::IncorrectNum(s)));

    num
        .map(|r| r.map(|n| Json::Num(n)))
        .map_err(|_| JsonError::UndefinedError)
        .apply(input)
        .into()
}

#[test]
fn json() {
    let json = read_num("-12").result().unwrap();
    assert_eq!(json, Json::Num(-12));
}
