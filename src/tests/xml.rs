use crate::{Parse, par, pred_fn, stringed_par, any, BoxedStrParser};

#[derive(Debug, PartialEq)]
struct Xml {
    name: String,
    attrs: Vec<(String, String)>,
    inner: Vec<Xml>,
}

fn parser<'p>() -> BoxedStrParser<'p, Vec<Xml>> {
    let space = (par(' ') | '\n' | '\t') * ..;
    let digit = pred_fn(pattern!('0'..='9'));
    let letter = pred_fn(pattern!('a'..='z')) | pred_fn(pattern!('A'..='Z'));
    let ident_begin = par(letter | '_');
    let ident = stringed_par(ident_begin) & (ident_begin | digit | '-') * ..;

    let quote = par('"') >> (par("\\\"") | any()).until('"').map(|(s, _)| s);
    let attr = par((ident, par('=') >> quote));
    let attrs = (space >> attr) ^ ..;
    let inner = parser;

    let name = par('<') >> ident;
    let tag_attrs = space >> attrs << space << '>';

    let tag = par((name, tag_attrs, inner))
        .map(|(name, attrs, inner)| Xml {
            name,
            attrs,
            inner,
        })
        .and_then(move |xml: &Xml| space >> "</" >> xml.name.clone() << '>')
        .map(|(x, _)| x);

    ((space >> tag) ^ ..).boxed()
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
