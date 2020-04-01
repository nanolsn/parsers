# parsers
[![Latest Version]][crates.io]

## Description
The library of parser combinators.
Using rules, you can express a grammar of any language.
Here is common rules such as "latin" or "any". Also rule combinators: "or", "cat", "range" and etc.

## Common rules
| Type  | Match                                 | Constructor |
|:------|:--------------------------------------|:------------|
| Dec   | `0...9`                               | `dec()`     |
| Bin   | `0` or `1`                            | `bin()`     |
| Oct   | `0...7`                               | `oct()`     |
| Hex   | `0...9` or `A...F`                    | `hex()`     |
| Space | ` `                                   | `space()`   |
| White | `\r\n` or ` ` or `\n` or `\r` or `\t` | `white()`   |
| Nl    | `\r\n` or `\n` or `\r`                | `nl()`      |
| Latin | `a...z` or `A...Z`                    | `latin()`   |
| Any   | Any char                              | `any()`     |
| End   | Checks rest input is empty            | `end()`     |

## Combinators
| Type      | Description                                             | Operator / Constructor  |
|:----------|:--------------------------------------------------------|:------------------------|
| Fst       | Parses *x* and *y* then return *x*                      | `x << y`                |
| Snd       | Parses *x* and *y* then return *y*                      | `x >> y`                |
| Or        | Parses *x* or *y*                                       | <code>x &#124; y</code> |
| Cat       | Parses *x* and *y* then concat result                   | `x.cat(y)`              |
| AndThen   | Parses *x* and then applies result to *f* and parses it | `x.and_then(f)`         |
| OrElse    | Parses *x* or else applies error to *f* and parses it   | `x.or_else(f)`          |
| CharRange | Parses char range                                       | `char_range(a..=b)`     |
| Not       | Parses *x* and reverse result                           | `!x`                    |
| Opt       | Makes rule *x* optional and return `Option`             | `x.opt()`               |
| OrDefault | Makes rule *x* optional and return default              | `x.or_default()`        |
| Pred      | Applies predicate *p* to char and return it if true     | `x.pred(p)`             |
| Range     | Parses *x* multiple times                               | `x.range(0..n)`         |
| Until     | Parses *x* until *y*                                    | `x.until(y)`            |
| Ret       | Always return value *v*                                 | `ret(v)`                |
| RetErr    | Always return error *e*                                 | `ret_err(e)`            |
| Map       | Parses *x* then apply function *f* to successful result | `x.map(f)`              |
| MapErr    | Parses *x* then apply function *f* to error result      | `x.map_err(f)`          |
| Into      | Parses *x* and convert result to `Type`                 | `into::<Type>(x)`       |
