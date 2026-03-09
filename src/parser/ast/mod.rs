pub mod expressions;
pub mod declarations;
pub mod statements;

pub use std::fmt;

pub use expressions::*;
pub use declarations::*;
pub use statements::*;

// --- Display Helpers ---

pub fn fmt_vector<T: fmt::Display>(vec: &[T]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for (i, item) in vec.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write!(out, "{item}").unwrap();
    }
    out
}

#[allow(clippy::missing_errors_doc)]
//Write each item on its own indented line.
pub fn pretty_list(
    f: &mut impl fmt::Write,
    items: &[impl fmt::Display],
    indent: &str,
) -> fmt::Result {
    if items.is_empty() {
        return writeln!(f, "{indent}(none)");
    }

    for item in items {
        let text = item.to_string();
        if text.is_empty() {
            continue;
        }
        for (i, line) in text.lines().enumerate() {
            if i == 0 {
                writeln!(f, "{indent}• {line}")?;
            } else {
                writeln!(f, "{indent}  {line}")?;
            }
        }
    }
    Ok(())
}
