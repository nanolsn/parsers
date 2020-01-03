## Description
The library of parser combinators.
Instead of using "parsers" everywhere, here's some different conceptions like "parsers" and "rules".
Using rules, you can express a grammar of any language. Also using a parser to parse an input and get a result.
Here is common rules such as "alpha" or "any". Also rule combinators: "or", "concat", "range" and etc.

## Common rules
| Type     | Match                                   | Constructor   |
|:---------|:----------------------------------------|:--------------|
| Digit    | `0...9`                                 | `digit()`     |
| HexDigit | `0...9` or `A...F`                      | `hex_digit()` |
| Space    | ` `                                     | `space()`     |
| White    | `\r\n` or ` ` or `\n` or `\r` or `\t`   | `white()`     |
| NewLine  | `\r\n` or `\n`                          | `new_line()`  |
| Alpha    | `a...z` or `A...Z`                      | `alpha()`     |
| Any      | Any char                                | `any()`       |

## Combinators
| Type      | Description                                       | Operator / Constructor  |
|:----------|:--------------------------------------------------|:------------------------|
| First     | Parses *a* and *b* then return *a*                | `a << b`                |
| Second    | Parses *a* and *b* then return *b*                | `a >> b`                |
| Or        | Parses *a* or *b*                                 | <code>a &#124; b</code> |
| Concat    | Parses *a* and *b* then concat result to `String` | `a & b`                 |
| End       | Checks rest input is empty and return `""`        | `end()`                 |
| CharRange | Parses inclusive char range                       | `char_range(a..=b)`     |
| Not       | Parses *a* and reverse `Result`                   | `!a`                    |
| Opt       | Makes rule *a* optional and return `Option`       | `a.opt()`               |
| OrEmpty   | Makes rule *a* optional and return `""`           | `a.or_empty()`          |
| Pred      | Applies predicate *p* to char                     | `a.pred(p)`             |
| Range     | Parses *a* multiple times and return `String`     | `a * (0..n)`            |
| RangeVec  | Parses *a* multiple times and return `Vec`        | `a ^ (0..n)`            |
| Until     | Parses *a* until *b* and return `String`          | `a.until(b)`            |
| UntilVec  | Parses *a* until *b* and return `Vec`             | `a.until_vec(b)`        |
| Ret       | Always return value *v*                           | `ret(v)`                |
| RetErr    | Always return error *e*                           | `ret_err(e)`            |
| BoxedRule | Wraps rule *a* into `Box`                         | `boxed(a)`              |

## Transformations
| Type      | Description                                             | Constructor            |
|:----------|:--------------------------------------------------------|:-----------------------|
| Map       | Parses *a* then apply function *f* to successful result | `a.map(f)`             |
| MapErr    | Parses *a* then apply function *f* to error result      | `a.map_err(f)`         |
| StringRes | Parses *a* and convert result to `String`               | `string_res(a)`        |

## Example

This is a short example of numbers parser. On output of parser you will get `Vec` of enum `Number`:
```rust
#[derive(PartialEq, Debug)]
enum Number {
    Bin(i64),
    Dec(i64),
    Hex(i64),
}

use Number::*;
```

In order to parse number, it needs define a function for any kind of number (for bin, dec, hex format).
Then combine them in other function like `any_num`, which parses enum `Number` and then `numbers` function, which parses multiple separated numbers.
```rust
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
```

At the final step, make a `Parser` and pass an input.
As result you will get `Vec<Number>`, if input string was correct.
```rust
#[test]
fn numbers_parser() {
    let parser = Parser::new(" 0xFF -25 0b0101 ");

    assert_eq!(
        parser.parse_result(numbers),
        Ok(vec![Hex(255), Dec(-25), Bin(5)]),
    );
}
```
