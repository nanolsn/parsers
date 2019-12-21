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
