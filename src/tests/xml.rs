use crate::{Parse, par, pred_fn, stringed_par, any, BoxedParser, boxed};

#[derive(Debug, PartialEq)]
struct Xml {
    name: String,
    attrs: Vec<(String, String)>,
    inner: Vec<Xml>,
}

fn space<'i>() -> BoxedParser<'i, String> {
    boxed((par(' ') | '\n' | '\t') * ..)
}

fn digit<'i>() -> BoxedParser<'i, &'i str> {
    pred_fn(pattern!('0'..='9')).boxed()
}

fn letter<'i>() -> BoxedParser<'i, &'i str> {
    boxed(pred_fn(pattern!('a'..='z')) | pred_fn(pattern!('A'..='Z')))
}

fn ident_begin<'i>() -> BoxedParser<'i, &'i str> {
    boxed(letter() | '_')
}

fn ident<'i>() -> BoxedParser<'i, String> {
    (stringed_par(ident_begin()) & (ident_begin() | digit() | '-') * ..).boxed()
}

fn quote<'i>() -> BoxedParser<'i, String> {
    (par('"') >> (par("\\\"") | any()).until('"').map(|(s, _)| s)).boxed()
}

fn attr<'i>() -> BoxedParser<'i, (String, String)> {
    par((ident(), par('=') >> quote())).boxed()
}

fn attrs<'i>() -> BoxedParser<'i, Vec<(String, String)>> {
    boxed((space() >> attr()) ^ ..)
}

fn name<'i>() -> BoxedParser<'i, String> {
    boxed(par('<') >> ident())
}

fn tag_attrs<'i>() -> BoxedParser<'i, Vec<(String, String)>> {
    boxed(space() >> attrs() << space() << '>')
}

fn closed_name<'i>() -> BoxedParser<'i, String> {
    boxed(space() >> "</" >> ident() << '>')
}

fn parse_xml<'i>() -> BoxedParser<'i, Xml> {
    let xml = par((name(), tag_attrs(), parser, closed_name()))
        .pred(|(name, _, _, closed_name)| name == closed_name)
        .map(|(name, attrs, inner, _)| Xml {
            name,
            attrs,
            inner,
        });
    xml.boxed()
}

fn parser<'i>() -> BoxedParser<'i, Vec<Xml>> {
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
