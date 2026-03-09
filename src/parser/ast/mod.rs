pub mod expressions;
pub mod declarations;
pub mod statements;

pub use std::fmt;

pub use expressions::*;
pub use declarations::*;
pub use statements::*;

// --- XML Helpers ---

pub fn xml_open_tag(out: &mut String, tag: &str, indent: usize) {
    xml_indent(out, indent);
    out.push('<');
    out.push_str(tag);
    out.push_str(">\n");
}

pub fn xml_close_tag(out: &mut String, tag: &str, indent: usize) {
    xml_indent(out, indent);
    out.push_str("</");
    out.push_str(tag);
    out.push_str(">\n");
}

pub fn xml_terminal(out: &mut String, tag: &str, value: &str, indent: usize) {
    xml_indent(out, indent);
    out.push('<');
    out.push_str(tag);
    out.push_str("> ");
    out.push_str(value);
    out.push_str(" </");
    out.push_str(tag);
    out.push_str(">\n");
}

pub fn xml_keyword(out: &mut String, kw: &str, indent: usize) {
    xml_terminal(out, "keyword", kw, indent);
}

pub fn xml_identifier(out: &mut String, name: &str, indent: usize) {
    xml_terminal(out, "identifier", name, indent);
}

pub fn xml_symbol(out: &mut String, c: char, indent: usize) {
    let escaped: String = match c {
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '&' => "&amp;".into(),
        '"' => "&quot;".into(),
        other => {
            let mut s = String::with_capacity(1);
            s.push(other);
            s
        }
    };
    xml_terminal(out, "symbol", &escaped, indent);
}

pub fn xml_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}
