use crate::parser::{parser, Turbo};
use chumsky::{error::Simple, Parser};
use std::fs;

mod ast;
mod html;
mod parser;

use std::fmt;

pub use ast::TurboTree;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TurboTextMod {
    Bold,
    Cursive,
    Strike,
    Underline,
    Code,
    Sup,
    Sub,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListKind {
    Numbered,
    AlphabetUpper,
    AlphabetLower,
    RomanUpper,
    RomanLower,
    Unordered(Option<usize>),
}

impl fmt::Display for ListKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ListKind::Numbered => "Unordered".to_string(),
            ListKind::AlphabetUpper => "Alphabet Upper".to_string(),
            ListKind::AlphabetLower => "Alphabet Lower".to_string(),
            ListKind::RomanUpper => "Roman Upper".to_string(),
            ListKind::RomanLower => "Roman Lower".to_string(),
            ListKind::Unordered(_) => "Unordered".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub lang: Lang,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lang {
    Turbo,
    KaTeX,
    Rust,
    Nim,
    Python,
    C,
    CPP,
    Other(String),
}

pub fn parse_file(path: &str) -> Turbo {
    let path = if path.ends_with(".tmd") {
        path.to_string()
    } else {
        format!("{}.tmd", path)
    };
    let mut content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => panic!("{e}"),
    };
    if !content.ends_with("\n") {
        content.push('\n')
    }

    let (turbo, _errors) = parse(&content);

    turbo
}

pub fn parse_string(content: &str) -> Turbo {
    let content = if !content.ends_with("\n") {
        let mut content = content.to_string();
        content.push_str("\n");
        content
    } else {
        content.to_string()
    };

    let (turbo, _errors) = parse(&content);

    turbo
}

pub fn parse(content: &str) -> (Turbo, Vec<Simple<char>>) {
    let (turbo, errors) = parser().parse_recovery(content);
    (turbo.unwrap(), errors)
}

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        use Lang::*;
        let res = match value {
            "turbo" => Turbo,
            "katex" => KaTeX,
            "math" => KaTeX,
            "rust" => Rust,
            "nim" => Nim,
            "python" => Python,
            "c" => C,
            "cpp" => CPP,
            "c++" => CPP,
            _ => Other(value.to_string()),
        };
        res
    }
}

impl Lang {
    pub fn as_str(&self) -> &str {
        return match self {
            Lang::Turbo => "turbo",
            Lang::KaTeX => "katex",
            Lang::Rust => "rust",
            Lang::Nim => "nim",
            Lang::Python => "python",
            Lang::C => "c",
            Lang::CPP => "cpp",
            Lang::Other(s) => s.as_str(),
        };
    }
}

impl Turbo {
    pub fn root(&self) -> &Vec<Turbo> {
        match self {
            Turbo::Root(vec) => vec,
            _ => panic!("illegal call"),
        }
    }
}

// impl Display for Turbo {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_pretty_string())
//     }
// }

// impl Turbo {
//     pub fn to_pretty_string(&self) -> String {
//         let mut buffer = String::new();
//         self.pretty_string_internal(0, &mut buffer);
//         buffer
//     }
//
//     fn pretty_string_internal(&self, layer: u32, buffer: &mut String) {
//         for _ in 0..layer {
//             buffer.push_str("  ")
//         }
//         match self {
//             TurboMd::Invalid => buffer.push_str("Invalid\n"),
//             TurboMd::Heading(size, content) => {
//                 buffer.push_str("Heading:\n");
//                 for _ in 0..layer + 1 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str(&format!("size: {}\n", size));
//                 for _ in 0..layer + 1 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str(&format!("content:\n"));
//                 for _ in 0..layer + 2 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str("\"");
//                 for inline in content {
//                     inline.pretty_string_internal(layer + 2, false, buffer);
//                 }
//                 buffer.push_str("\"");
//                 buffer.push_str("\n")
//             }
//             TurboMd::Quote => {
//                 todo!()
//             }
//             TurboMd::Line(content) => {
//                 buffer.push_str("Line:\n");
//                 for _ in 0..layer + 1 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str("\"");
//                 for inline in content {
//                     inline.pretty_string_internal(layer + 1, false, buffer);
//                 }
//                 buffer.push_str("\"");
//                 buffer.push_str("\n")
//             }
//             TurboMd::Comment(_content) => {
//                 todo!()
//             }
//             TurboMd::Root(content) => {
//                 buffer.push_str("Root:\n");
//                 for part in content {
//                     part.pretty_string_internal(layer + 1, buffer);
//                 }
//             }
//             TurboMd::Code(lang, body) => {
//                 buffer.push_str("Code:\n");
//                 for _ in 0..layer + 1 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str(&format!("lang: {:?}\n", lang));
//                 for _ in 0..layer + 1 {
//                     buffer.push_str("  ")
//                 }
//                 buffer.push_str(&format!("body:\n"));
//
//                 for line in body.lines() {
//                     for _ in 0..layer + 2 {
//                         buffer.push_str("  ")
//                     }
//                     buffer.push_str(line);
//                     buffer.push('\n')
//                 }
//                 buffer.push_str("\n")
//             }
//             TurboMd::List(_kind, _elements) => {}
//             TurboMd::ListElement(_content) => {}
//         }
//     }
// }
//
// impl TurboInline {
//     pub fn pretty_string_internal(&self, layer: u32, should_layer: bool, buffer: &mut String) {
//         if should_layer {
//             for _ in 0..layer {
//                 buffer.push_str("  ")
//             }
//         }
//         match self {
//             TurboInline::TextContainer(modifier, content) => {
//                 buffer.push_str(&format!("({:?})[", modifier));
//                 content.pretty_string_internal(layer + 1, false, buffer);
//                 buffer.push_str("]");
//             }
//             TurboInline::Text(content) => buffer.push_str(content),
//             TurboInline::TextNewLine => buffer.push_str(" [NewLine] "),
//         }
//     }
// }
