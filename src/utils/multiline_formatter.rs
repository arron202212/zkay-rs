use textwrap::{dedent, indent};
// from typing import Union, List

// class MultiLineFormatter
pub struct MultiLineFormatter {
    pub text: String,
    current_indent: String,
    indent_str: String,
}
impl MultiLineFormatter {
    // """
    // \\* operator -> add de-dented text (+ \\\\n), if operand is a list -> add \\\\n-joined elements

    // % operator -> add de-dented text (+ \\\\n), if operand is a list -> add ,-joined elements

    // / operator -> increase indentation level and add text (+ \\\\n)

    // // operator -> decrease indentation level and add text (+ \\\\n)
    // """
    pub fn new(indent_str: &str) -> Self {
        let indent_str = if indent_str.is_empty() {
            " ".repeat(4)
        } else {
            indent_str.to_owned()
        };
        Self {
            text: String::new(),
            current_indent: String::new(),
            indent_str,
        }
    }

    pub fn mul(&mut self, other: (Option<String>, Option<Vec<String>>)) -> Self {
        if let (Some(other), None) = other {
            self.append(other)
        } else if let (None, Some(other)) = other {
            self.text += "\n";
            self.append_lines(other)
        }
    }

    pub fn modular(&mut self, other: (Option<String>, Option<Vec<String>>)) -> Self {
        if let (Some(str), None) = other {
            self.append(other, ", ")
        } else if let (Some(str), None) = other {
            self.append_lines(other, ", ")
        } else {
            self.clone()
        }
    }

    pub fn truediv(&mut self, other: String) -> Self {
        if other {
            self.indent().mul(other)
        } else {
            self.indent()
        }
    }

    pub fn floordiv(&mut self, other: &str) -> Self {
        self.dedent().mul(other)
    }

    pub fn str(self) -> String {
        format!("{}\n", self.text.trim())
    }

    pub fn append(&mut self, txt: String, sep: &str) -> Self {
        let sep = if sep.is_empty() { "\n" } else { sep };
        self.text += sep;
        if !txt.is_empty() {
            self.text += indent(dedent(txt), self.current_indent);
        }
        self.clone()
    }

    pub fn append_lines(&mut self, lines: Vec<&str>, sep: &str) -> Self {
        let sep = if sep.is_empty() { "\n" } else { sep };
        self.text += lines
            .inito_iter()
            .filter_map(|t| {
                if t.is_empty() {
                    None
                } else {
                    indent(
                        dedent(if t != "\n" { t } else { String::new() }),
                        self.current_indent,
                    )
                }
            })
            .collect::<Vec<_>>()
            .join(sep);
        self.clone()
    }

    pub fn indent(&mut self) -> Self {
        self.current_indent += self.indent_str;
        self.clone()
    }

    pub fn dedent(&self) -> Self {
        assert!(self.current_indent.len() >= self.indent_str.len());
        self.current_indent =
            self.current_indent[..self.current_indent.len() - self.indent_str.len()];
        self.clone()
    }
}
