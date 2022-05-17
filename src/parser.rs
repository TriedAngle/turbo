use crate::{Code, Lang, ListKind, Turbo, TurboInlineRaw, TurboTextMod};
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Turbo, Error = Simple<char>> {
    let plain_text = filter(|c| {
        *c != '#'
            && *c != '*'
            && *c != '_'
            && *c != '~'
            && *c != '`'
            && *c != '['
            && *c != '\n'
            && *c != '\\'
    })
    .repeated()
    .at_least(1)
    .collect::<String>()
    .map(TurboInlineRaw::Text);

    let extended_backslash_text = filter(|c| *c != '\n' && *c != '{' && *c != '}')
        .repeated()
        .collect::<String>()
        .map(TurboInlineRaw::Text);

    let number = just::<_, char, Simple<char>>('-')
        .or_not()
        .then(text::digits(10))
        .map(|(neg, number)| {
            if neg.is_some() {
                -number.parse::<isize>().unwrap()
            } else {
                number.parse::<isize>().unwrap()
            }
        });

    let text_modifier = choice((
        just('*').to(TurboTextMod::Bold),
        just('_').to(TurboTextMod::Cursive),
        just('~').to(TurboTextMod::Strike),
        just('`').to(TurboTextMod::Code),
    ))
    .map(TurboInlineRaw::ModFlag);

    let backslash_extended = just('\\')
        .ignore_then(just('{'))
        .ignore_then(extended_backslash_text)
        .then_ignore(just('}'));

    let backslash = just('\\')
        .ignore_then(one_of("*_~`-[\\"))
        .map(|val| TurboInlineRaw::Text(val.to_string()));

    let new_line = just('\\').then(just('\n')).to(TurboInlineRaw::NewLine);

    let link = filter(|c| *c != '\n' && *c != ']')
        .repeated()
        .collect::<String>()
        .map(|val| if val == "" { None } else { Some(val) })
        .delimited_by(just('['), just(']'))
        .then(
            filter(|c| *c != '\n' && *c != ')')
                .repeated()
                .collect::<String>()
                .delimited_by(just('('), just(')')),
        )
        .map(|(alias, address)| TurboInlineRaw::Link { alias, address });

    let inline = choice((
        backslash,
        backslash_extended,
        link,
        text_modifier,
        new_line,
        plain_text,
    ));

    let whitespace = just(' ').repeated().collect::<String>().map(|s| s.len());

    let text_line = inline.repeated().then_ignore(text::newline());

    let ident_text_line = whitespace
        .then(text_line.clone())
        .map(|(ident, text)| Turbo::Line { ident, text });

    let header_tag = filter(|c| *c == '#')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .then_ignore(just(' ').or_not())
        .map(|tag| tag.len());

    let header = whitespace
        .clone()
        .then(header_tag.then(text_line.clone()))
        .map(|(ident, (size, text))| Turbo::Header { ident, size, text });

    let hr = just('-')
        .repeated()
        .at_least(3)
        .ignore_then(just('\n'))
        .to(Turbo::Horizontal);

    let empty = just('\n').to(Turbo::Empty);

    let list_tag = choice((
        just('-').then(just(' ').or_not()).to(ListKind::Unordered),
        just('-')
            .ignore_then(just(' ').or_not())
            .ignore_then(number)
            .then_ignore(just(' ').or_not())
            .map(ListKind::Ordered),
    ));

    let check = choice((just('x'), just(' ')))
        .delimited_by(just('['), just(']'))
        .then_ignore(just(' ').or_not())
        .map(|x| x == 'x');

    let code_start = just(':')
        .repeated()
        .exactly(3)
        .ignore_then(just(' ').repeated())
        .ignore_then(
            filter(|c| *c != '\n')
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .then_ignore(just('\n'));

    let code_end = just(':').repeated().exactly(3).then(just('\n').or_not());

    let code = code_start
        .then(take_until(code_end))
        .map(|(lang, (code, _))| Turbo::Code {
            ident: None,
            code: Code {
                lang: Lang::from(lang.as_ref()),
                code: code.iter().collect::<String>(),
            },
        });

    let code_ident =
        whitespace
            .then(code_start.then(take_until(code_end)))
            .map(|(ident, (lang, (code, _)))| Turbo::Code {
                ident: Some(ident),
                code: Code {
                    lang: Lang::from(lang.as_ref()),
                    code: code.iter().collect::<String>(),
                },
            });

    let list_element_start = whitespace
        .then(list_tag)
        .then(check.or_not())
        .then(choice((
            code,
            text_line.map(|text| Turbo::Line { ident: 0, text }),
        )))
        .map(|(((ident, kind), check), content)| Turbo::ListElemStart {
            ident,
            kind,
            check,
            content: vec![content],
        });

    let include = whitespace
        .then(
            just('@')
                .ignore_then(
                    any()
                        .repeated()
                        .collect::<String>()
                        .delimited_by(just('['), just(']')),
                )
                .then_ignore(just('\n')),
        )
        .map(|(ident, path)| Turbo::Include { ident, path });

    choice((
        header,
        hr,
        empty,
        code_ident,
        include,
        list_element_start,
        ident_text_line,
    ))
    .repeated()
    .map(Turbo::Root)
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use chumsky::Parser;

    const INPUT: &str = r#"
# Test
Hello *World*
----
This is _~text~_\
and more \* text
wow stars \{***}
just [google](https://google.com) it lol

## multiline\
heading uwu

::: rust
fn main() {
    println!("Hello World!")
}
:::
- list element start
    further text
    - [ ] a unchecked check
    - [x] a checked check
    - ok this is getting nested
- and even more list
@[chapter1]
"#;

    #[test]
    fn parse() {
        println!("{:?}", parser().parse(INPUT));
        // println!("{:?}", parser().parse("## Test\n"));
    }

    #[test]
    fn test() {
        // let test = filter::<_, _, Simple<char>>(|c| *c != '\n' && *c != ']')
        //     .repeated()
        //     .collect::<String>()
        //     .map(|val| if val == "" { None } else { Some(val) })
        //     .delimited_by(just('['), just(']'))
        //     .then(
        //         filter(|c| *c != '\n' && *c != ')')
        //             .repeated()
        //             .collect::<String>()
        //             .delimited_by(just('('), just(')'))
        //     )
        //     .map(|(alias, address)| TurboInline::Link {alias, address});
        //
        // println!("{:?}", test.parse("[google](https://google.com)"));
    }
}
