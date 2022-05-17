use crate::{Code, ListKind, Turbo, TurboInlineRaw, TurboTextMod, TurboTextRaw};
use chumsky::prelude::*;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TurboTree {
    Root {
        content: Vec<TurboTree>,
    },
    Text(TurboText),
    Heading {
        size: usize,
        text: TurboText,
    },
    List {
        kind: ListKind,
        items: Vec<TurboTree>,
    },
    ListItem {
        check: Option<bool>,
        items: Vec<TurboTree>,
    },
    Code(Code),
    Horizontal,
    Empty,
    Include(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TurboText {
    TextContainer(Vec<TurboText>),
    Mod {
        kind: TurboTextMod,
        text: Vec<TurboText>,
    },
    Link {
        alias: Option<String>,
        address: String,
    },
    Plain(String),
    NewLine,
}

impl TurboTree {
    fn root(&mut self) -> &mut Vec<TurboTree> {
        match self {
            TurboTree::Root { content } => (content),
            _ => panic!("illegal call"),
        }
    }

    fn is_list_item(&self) -> bool {
        match self {
            TurboTree::ListItem { .. } => true,
            _ => false,
        }
    }
}

impl TurboText {
    pub fn get_vec_mut(&mut self) -> &mut Vec<TurboText> {
        match self {
            TurboText::TextContainer(vec) => vec,
            TurboText::Mod { text, .. } => text,
            _ => panic!("illegal call"),
        }
    }
}

impl TurboTree {
    pub fn generate(parse: Turbo) -> Self {
        let mut content = vec![];
        let root = parse.root();
        let mut idx = 0;
        while idx < root.len() {
            let (next_idx, next) = generate_recursive(root, idx, 0, None);
            content.push(next.unwrap());
            idx = next_idx;
        }

        let tree = TurboTree::Root {
            content,
        };
        tree
    }

    pub fn get_vec(&self) -> &Vec<Self> {
        match self {
            TurboTree::Root { content, .. } => content,
            TurboTree::List { items, .. } => items,
            TurboTree::ListItem { items, .. } => items,
            _ => panic!("Illegal Call")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListSetting {
    pub kind: ListKind,
    pub nesting_counter: usize,
}

fn generate_recursive(
    turbo: &Vec<Turbo>,
    mut current: usize,
    current_ident: usize,
    list_setting: Option<ListSetting>,
) -> (usize, Option<TurboTree>) {
    let item = match &turbo[current] {
        Turbo::Header { ident, size, text } => {
            if list_setting.is_some() {
                if *ident <= current_ident {
                    return (current + 1, None);
                }
            }

            TurboTree::Heading {
                size: *size,
                text: turbo_text(text),
            }
        }
        Turbo::Horizontal => TurboTree::Horizontal,

        Turbo::Empty => TurboTree::Empty,
        Turbo::Line { ident, text } => {
            if list_setting.is_some() {
                if *ident <= current_ident {
                    return (current + 1, None);
                }
            } else {
                if *ident != current_ident {
                    return (current + 1, None);
                }
            }

            let mut text = turbo_text(text);

            let mut idx = current + 1;
            while idx < turbo.len() {
                if let Some((next_ident, next_text)) = turbo[idx].line() {
                    if next_ident == ident {
                        text.get_vec_mut().push(TurboText::Plain(" ".to_string()));
                        turbo_text_extend(&mut text, next_text);
                    } else {
                        break;
                    }
                    idx += 1;
                } else {
                    break;
                }
            }
            current = idx;
            return (current, Some(TurboTree::Text(text)));
        }
        Turbo::ListElemStart {
            ident,
            kind,
            check,
            content,
        } => {
            if let Some(setting) = &list_setting {
                if *ident == current_ident && setting.nesting_counter > 1 {
                    return (current, None);
                }
                if *ident < current_ident {
                    return (current, None);
                }
            }
            let mut items = vec![];

            let (_, item) = generate_recursive(content, 0, 0, None);
            items.push(item.unwrap());

            let nc = if let Some(setting) = &list_setting {
                if *ident == current_ident && &setting.kind == kind {
                    setting.nesting_counter
                } else {
                    0
                }
            } else { 0 };

            let mut idx = current + 1;
            while idx < turbo.len() {
                let (next_idx, next) = generate_recursive(
                    turbo,
                    idx,
                    *ident,
                    Some(ListSetting { kind: kind.clone(), nesting_counter: nc + 1}),
                );
                if let Some(next) = next {
                    items.push(next);
                    idx = next_idx
                } else {
                    break;
                }
            }

            current = idx;

            if let Some(_) = &list_setting {
                if *ident == current_ident {
                    return (current, Some(TurboTree::ListItem {check: *check, items}))
                }
            }

            let mut split_index = 0;
            for item in &items {
                if item.is_list_item() { break; }
                split_index += 1;
            }
            let mut rest = items.split_off(split_index);
            let list_item = TurboTree::ListItem { check: *check, items };
            rest.insert(0, list_item);
            return (current, Some(TurboTree::List {kind: kind.clone(), items: rest}))
        }

        Turbo::Code { ident, code } => {
            if list_setting.is_some() {
                if let Some(ident) = ident {
                    if *ident <= current_ident {
                        return (current, None);
                    }
                }
            }
            TurboTree::Code(code.clone())
        }

        Turbo::Include { ident, path } => {
            if list_setting.is_some() {
                if *ident <= current_ident {
                    return (current, None);
                }
            }
            TurboTree::Include(path.clone())
        }
        Turbo::Root(_) => {
            panic!("Illegal Root")
        }
    };

    (current + 1, Some(item))
}

fn turbo_text_extend(to_extend: &mut TurboText, raw: &TurboTextRaw) {
    turbo_text_recursive(to_extend, raw, 0, &mut HashSet::new());
}

fn turbo_text(raw: &TurboTextRaw) -> TurboText {
    let mut tt = TurboText::TextContainer(vec![]);
    turbo_text_recursive(&mut tt, raw, 0, &mut HashSet::new());
    tt
}

fn turbo_text_recursive(
    tt: &mut TurboText,
    text: &TurboTextRaw,
    mut current: usize,
    stats: &mut HashSet<TurboTextMod>,
) -> usize {
    while current < text.len() {
        match &text[current] {
            TurboInlineRaw::NewLine => {
                let vec = tt.get_vec_mut();
                vec.push(TurboText::NewLine);
            }
            TurboInlineRaw::ModFlag(m) => {
                if !stats.contains(m) {
                    stats.insert(*m);
                    let vec = tt.get_vec_mut();
                    let mut modifier = TurboText::Mod {
                        kind: *m,
                        text: vec![],
                    };
                    current = turbo_text_recursive(&mut modifier, text, current + 1, stats);
                    vec.push(modifier);
                } else {
                    stats.remove(m);
                    return current;
                }
            }
            TurboInlineRaw::Link { alias, address } => {
                tt.get_vec_mut().push(TurboText::Link {
                    alias: alias.clone(),
                    address: address.clone(),
                });
            }
            TurboInlineRaw::Text(p) => {
                tt.get_vec_mut().push(TurboText::Plain(p.clone()));
            }
        }
        current += 1
    }
    current
}


impl fmt::Display for TurboTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.pretty_string(0))
    }
}

impl TurboTree {
    pub fn pretty_string(&self, level: usize) -> String {
        let mut buffer = String::new();
        let whitespace = |level: usize| (0..level * 2).map(|val| ' ').collect::<String>();
        match self {
            TurboTree::Root { content } => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Root:\n");
                for part in content {
                    buffer.push_str(&part.pretty_string(level + 1))
                }
            }
            TurboTree::Text(text) => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Text:\n");
                buffer.push_str(&format!("{}{:?}", whitespace(level + 1), text));
                buffer.push_str("\n");
            }
            TurboTree::Heading { size, text } => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Heading:\n");
                buffer.push_str(&format!("{}size: {}\n", whitespace(level + 1), size));
                buffer.push_str(&format!("{}text:\n", whitespace(level + 1)));
                buffer.push_str(&format!("{}{:?}", whitespace(level + 1), text));
                buffer.push_str("\n");
            }
            TurboTree::List { kind, items } => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("List:\n");
                buffer.push_str(&format!("{}kind: {}\n", whitespace(level + 1), kind));
                buffer.push_str(&format!("{}items:\n", whitespace(level + 1)));
                for item in items {
                    buffer.push_str(&item.pretty_string(level + 2));
                }
            }
            TurboTree::ListItem { check, items } => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("List Item:\n");
                buffer.push_str(&format!("{}check: {:?}\n", whitespace(level + 1), check));
                buffer.push_str(&format!("{}items:\n", whitespace(level + 1)));
                for item in items {
                    buffer.push_str(&item.pretty_string(level + 2))
                }
            }
            TurboTree::Code(code) => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Code:\n");
                buffer.push_str(&format!("{}Lang: {:?}\n", whitespace(level + 1), code.lang));
                buffer.push_str(&format!("{}Content:\n", whitespace(level + 1)));
                for line in code.code.lines() {
                    buffer.push_str(&whitespace(level + 2));
                    buffer.push_str(line);
                    buffer.push('\n');
                }
            }
            TurboTree::Horizontal => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Horizontal\n");
            }
            TurboTree::Empty => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Empty:\n");
            }
            TurboTree::Include(path) => {
                buffer.push_str(&whitespace(level));
                buffer.push_str("Include:\n");
                buffer.push_str(&format!("{}check: {:?}\n", whitespace(level + 1), path));
            }
        }
        buffer
    }
}

#[cfg(test)]
mod ast_tests {
    use super::*;
    use crate::parser::parser;

    const INPUT: &str = "This is _a ~bad~ *super Test_ [hope](https://google.com) it _~*works*~_\n";

    #[test]
    fn test() {
        let parse = parser().parse(INPUT).unwrap();
        if let Turbo::Root(vec) = &parse {
            if let Turbo::Line { text, .. } = &vec[0] {
                let text = turbo_text(text);
                println!("{:?}", text);
            }
        }
    }
}
