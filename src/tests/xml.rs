use crate::{Parse, par, pred_fn, stringed_par, any, BoxedStrParser};

#[derive(Debug, PartialEq)]
struct Xml {
    name: String,
    attrs: Vec<(String, String)>,
    inner: Vec<Xml>,
}

fn space<'i>() -> BoxedStrParser<'i, String> {
    ((par(' ') | '\n' | '\t') * ..).boxed()
}

fn digit<'i>() -> BoxedStrParser<'i, &'i str> {
    pred_fn(pattern!('0'..='9')).boxed()
}

fn letter<'i>() -> BoxedStrParser<'i, &'i str> {
    (pred_fn(pattern!('a'..='z')) | pred_fn(pattern!('A'..='Z'))).boxed()
}

fn ident_begin<'i>() -> BoxedStrParser<'i, &'i str> {
    (par(letter() | '_')).boxed()
}

fn ident<'i>() -> BoxedStrParser<'i, String> {
    (stringed_par(ident_begin()) & (ident_begin() | digit() | '-') * ..).boxed()
}

fn quote<'i>() -> BoxedStrParser<'i, String> {
    (par('"') >> (par("\\\"") | any()).until('"').map(|(s, _)| s)).boxed()
}

fn attr<'i>() -> BoxedStrParser<'i, (String, String)> {
    par((ident(), par('=') >> quote())).boxed()
}

fn attrs<'i>() -> BoxedStrParser<'i, Vec<(String, String)>> {
    ((space() >> attr()) ^ ..).boxed()
}

fn name<'i>() -> BoxedStrParser<'i, String> {
    (par('<') >> ident()).boxed()
}

fn tag_attrs<'i>() -> BoxedStrParser<'i, Vec<(String, String)>> {
    (space() >> attrs() << space() << '>').boxed()
}

fn closed_name<'i>() -> impl Parse<'i, Res=String, Err=(), On=&'i str> {
    space() >> "</" >> ident() << '>'
}

fn parse_xml<'i>() -> impl Parse<'i, Res=Xml, Err=(), On=&'i str> {
    par((name(), tag_attrs(), parser, closed_name()))
        .pred(|(name, _, _, closed_name)| name == closed_name)
        .map(|(name, attrs, inner, _)| Xml {
            name,
            attrs,
            inner,
        })
}

fn parser<'i>() -> BoxedStrParser<'i, Vec<Xml>> {
    ((space() >> parse_xml()) ^ ..).boxed()
}

#[test]
fn xml() {
    let code = String::from(
        r#"
        <base type="data">
            <data index="1"  value="\"Hello\""></data>
            <input index="2"></input>
        </base>
        "#
    );

    assert_eq!(
        parser().parse_unwrap(code.as_str()),
        vec![Xml {
            name: "base".to_string(),
            attrs: vec![("type".to_string(), "data".to_string())],
            inner: vec![
                Xml {
                    name: "data".to_string(),
                    attrs: vec![
                        ("index".to_string(), "1".to_string()),
                        ("value".to_string(), "\\\"Hello\\\"".to_string()),
                    ],
                    inner: vec![],
                },
                Xml {
                    name: "input".to_string(),
                    attrs: vec![("index".to_string(), "2".to_string())],
                    inner: vec![],
                },
            ],
        }],
    );
}
