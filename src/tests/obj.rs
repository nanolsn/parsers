use crate::{Parser, space, white, digit, end, BoxedRule, boxed};
use crate::rule;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

fn spaces<'o>() -> BoxedRule<'o, String> {
    boxed(rule(space) * (1..))
}

fn whites<'o>() -> BoxedRule<'o, String> {
    boxed(rule(white) * ..)
}

fn float<'o>() -> BoxedRule<'o, f32> {
    (rule('-').or_empty() & rule(digit) * (1..) & '.' & rule(digit) * ..)
        .map(|s: String| s.parse::<f32>().unwrap())
        .boxed()
}

fn vector<'o>() -> BoxedRule<'o, Vertex> {
    (rule(whites) >> 'v' >> spaces() >>
        (float(), rule(spaces) >> float(), (rule(spaces) >> float()).opt())
    )
        .map(|(x, y, z): (f32, f32, Option<f32>)| {
            Vertex { x, y, z: z.unwrap_or(0.0) }
        })
        .boxed()
}

fn vectors<'o>() -> BoxedRule<'o, Vec<Vertex>> {
    boxed((rule(vector) << whites() ^ ..) << end())
}

#[test]
fn obj_parser() {
    let code = String::from(r"
        v 1.0 1.0 1.0
        v -1.0 1.0 1.0
    ");

    assert_eq!(
        Parser::new(code.as_str()).parse_result(vectors),
        Ok(vec![
            Vertex { x: 1.0, y: 1.0, z: 1.0 },
            Vertex { x: -1.0, y: 1.0, z: 1.0 },
        ]),
    );
}
