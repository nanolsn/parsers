use std::str::FromStr;

use crate::{
    apply::Apply,
    ruled::Ruled,
    rule::rule,
    expected::Expected,
    common::{
        white::white,
        dec::dec,
        bin::bin,
        oct::oct,
        hex::hex,
        any::any,
    },
};

#[derive(Debug, Eq, PartialEq)]
enum Json<'i> {
    Num(i64),
    Str(&'i str),
    Bool(bool),
    Array(Vec<Json<'i>>),
    Obj(Vec<(&'i str, Json<'i>)>),
}

#[derive(Debug, Eq, PartialEq)]
enum JsonError {
    IncorrectNum(String),
    Expected(Expected<'static>),
}

impl From<Expected<'static>> for JsonError {
    fn from(err: Expected<'static>) -> Self { JsonError::Expected(err) }
}

fn whites(code: &str) -> Ruled<&str, &str, Expected<'static>> { white().range(..).apply(code) }

fn str(code: &str) -> Ruled<&str, &str, JsonError> {
    (rule('"') >> (rule("\\\"") | any()).until('"'))
        .map_err(|e| JsonError::Expected(e))
        .map(|(s, _)| s)
        .apply(code)
}

fn read_num(code: &str) -> Ruled<&str, Json, JsonError> {
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
        .and_then(|r| r)
        .map(|n| Json::Num(n))
        .apply(code)
}

fn read_str(code: &str) -> Ruled<&str, Json, JsonError> {
    rule(str)
        .map(|s| Json::Str(s))
        .apply(code)
}

fn read_bool(code: &str) -> Ruled<&str, Json, JsonError> {
    let t = rule("true").map(|_| true);
    let f = rule("false").map(|_| false);

    (t | f)
        .map(|b| Json::Bool(b))
        .map_err(|e| JsonError::Expected(e))
        .apply(code)
}

//noinspection RsBorrowChecker
fn read_array(code: &str) -> Ruled<&str, Json, JsonError> {
    let el = rule(read_json);
    let els = (el << whites << ',').range(..).cat(el);
    let array = rule('[') >> els.or_default() << whites << ']';

    array
        .map(|els| Json::Array(els))
        .apply(code)
}

//noinspection RsBorrowChecker
fn read_obj(code: &str) -> Ruled<&str, Json, JsonError> {
    let key = rule(str);
    let value = rule(read_json);
    let el = rule((key, rule(whites) >> ':' >> value));
    let els = (el << whites << ',' << whites).range(..).cat(el);
    let obj = rule('{') >> whites >> els.or_default() << whites << '}';

    obj
        .map(|els| Json::Obj(els))
        .apply(code)
}

fn read_json(code: &str) -> Ruled<&str, Json, JsonError> {
    let json = rule(read_num) | read_str | read_bool | read_array | read_obj;

    (rule(whites) >> json)
        .apply(code)
}

#[test]
fn json() {
    let code = r#"
    {
        "is": true,
        "num": 12,
        "user": "nano",
        "data": [ 12 , "foo" ,  {} ],
        "profile\"":
        {
            "count": 0x0,
            "tag": "user",
            "indexes": [0,  0b1 , -2,3, 4]
        }
    }
    "#;

    let expected = Json::Obj(vec![
        ("is", Json::Bool(true)),
        ("num", Json::Num(12)),
        ("user", Json::Str("nano")),
        ("data", Json::Array(vec![
            Json::Num(12),
            Json::Str("foo"),
            Json::Obj(vec![]),
        ])),
        ("profile\\\"", Json::Obj(vec![
            ("count", Json::Num(0)),
            ("tag", Json::Str("user")),
            ("indexes", Json::Array(vec![
                Json::Num(0),
                Json::Num(1),
                Json::Num(-2),
                Json::Num(3),
                Json::Num(4),
            ])),
        ])),
    ]);

    let json = read_json(code).result().unwrap();
    assert_eq!(json, expected);
}
