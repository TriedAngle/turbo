use crate::ast::TurboText;
use crate::{ListKind, TurboTextMod, TurboTree};

impl TurboTree {
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        match self {
            TurboTree::Root { content } => content
                .iter()
                .for_each(|node| result.push_str(&node.to_html())),
            TurboTree::Text(text) => {
                result.push_str("<p>");
                result.push_str(&text.to_html());
                result.push_str("</p>\n")
            }
            TurboTree::Heading { size, text } => {
                result.push_str(&format!("<h{}>", size));
                result.push_str(&text.to_html());
                result.push_str(&format!("</h{}>\n", size))
            }
            TurboTree::List { kind, items } => {
                result.push_str(kind.to_html(false));
                result.push_str("\n");
                items
                    .iter()
                    .for_each(|node| result.push_str(&node.to_html()));
                result.push_str(kind.to_html(true));
                result.push_str("\n");
            }
            TurboTree::ListItem {
                id,
                check,
                label,
                items,
            } => {
                result.push_str("<li>\n");
                if let Some(check) = check {
                    if *check {
                        result.push_str(&format!(
                            "<input type=\"checkbox\" id=\"checkbox{id}\" checked=\"checked\"/>"
                        ));
                    } else {
                        result.push_str(&format!("<input type=\"checkbox\" id=\"checkbox{id}\"/>"));
                    }

                    result.push_str(&format!("<label for=\"checkbox{id}\">"));
                    if let Some(label) = label.as_ref() {
                        match label {
                            TurboTree::Text(text) => {
                                result.push_str(&text.to_html());
                            }
                            TurboTree::Heading { size, text } => {
                                result.push_str(&format!("<h{}>", size));
                                result.push_str(&text.to_html());
                                result.push_str(&format!("</h{}>", size))
                            }
                            _ => {
                                panic!("Only Text and Heading should be a list label")
                            }
                        }
                    }
                    result.push_str("</label>\n")
                } else {
                    if let Some(label) = label.as_ref() {
                        match label {
                            TurboTree::Text(text) => {
                                result.push_str("<p>");
                                result.push_str(&text.to_html());
                                result.push_str("</p>");
                            }
                            TurboTree::Heading { size, text } => {
                                result.push_str(&format!("<h{}>", size));
                                result.push_str(&text.to_html());
                                result.push_str(&format!("</h{}>", size))
                            }
                            _ => {
                                panic!("Only Text and Heading should be a list label")
                            }
                        }
                    }
                }
                items
                    .iter()
                    .for_each(|node| result.push_str(&node.to_html()));
                result.push_str("</li>\n")
            }
            TurboTree::Code(code) => {
                result.push_str("<pre>");
                result.push_str(&format!(
                    "<code class=\"language-{}\">\n",
                    code.lang.as_str()
                ));
                result.push_str(&format!("{}", &code.code));
                result.push_str("</code></pre>\n");
            }
            TurboTree::Horizontal => result.push_str("<hr/>\n"),
            TurboTree::Empty => {}
            TurboTree::Include(_) => {
                panic!("Include should be substituted in HTML stage")
            }
        }
        result
    }
}

impl TurboText {
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        match self {
            TurboText::TextContainer(text) => {
                text.iter()
                    .for_each(|node| result.push_str(&node.to_html()));
            }
            TurboText::Mod { kind, text } => {
                result.push_str(kind.to_html(false));
                text.iter()
                    .for_each(|node| result.push_str(&node.to_html()));
                result.push_str(kind.to_html(true));
            }
            TurboText::Link { alias, address } => {
                result.push_str(&format!("<a href=\"{}\">", address));
                if let Some(alias) = alias {
                    result.push_str(alias);
                } else {
                    result.push_str(address);
                }
                result.push_str("</a>");
            }
            TurboText::Plain(text) => {
                result.push_str(text);
            }
            TurboText::NewLine => result.push_str("<br/>"),
        }
        result
    }
}

impl TurboTextMod {
    pub fn to_html(&self, close: bool) -> &'static str {
        return match self {
            TurboTextMod::Bold => {
                if !close {
                    "<b>"
                } else {
                    "</b>"
                }
            }
            TurboTextMod::Cursive => {
                if !close {
                    "<i>"
                } else {
                    "</i>"
                }
            }
            TurboTextMod::Strike => {
                if !close {
                    "<del>"
                } else {
                    "</del>"
                }
            }
            TurboTextMod::Underline => {
                if !close {
                    "<ins>"
                } else {
                    "</ins>"
                }
            }
            TurboTextMod::Code => {
                if !close {
                    "<code>"
                } else {
                    "</code>"
                }
            }
            TurboTextMod::Sup => {
                if !close {
                    "<sup>"
                } else {
                    "</sup>"
                }
            }
            TurboTextMod::Sub => {
                if !close {
                    "<sub>"
                } else {
                    "</sub>"
                }
            }
        };
    }
}

impl ListKind {
    pub fn to_html(&self, close: bool) -> &'static str {
        return match self {
            ListKind::Numbered => {
                if !close {
                    "<ol type=\"1\">"
                } else {
                    "</ol>"
                }
            }
            ListKind::AlphabetUpper => {
                if !close {
                    "<ol type=\"1\">"
                } else {
                    "</ol>"
                }
            }
            ListKind::AlphabetLower => {
                if !close {
                    "<ol type=\"1\">"
                } else {
                    "</ol>"
                }
            }
            ListKind::RomanUpper => {
                if !close {
                    "<ol type=\"1\">"
                } else {
                    "</ol>"
                }
            }
            ListKind::RomanLower => {
                if !close {
                    "<ol type=\"1\">"
                } else {
                    "</ol>"
                }
            }
            ListKind::Unordered(ty) => {
                if close {
                    return "</ul>";
                }
                if ty.is_none() {
                    return "<ul>";
                }
                let ty = ty.unwrap();
                return match ty {
                    0 => "<ul style=\"list-style-type:none\">",
                    1 => "<ul style=\"list-style-type:circle\">",
                    2 => "<ul style=\"list-style-type:disc\">",
                    3 => "<ul style=\"list-style-type:square\">",
                    _ => {
                        panic!("unsupported type")
                    }
                };
            }
        };
    }
}
