// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub struct IndentWriter {
    buffer: String,
    level: usize,
    amount: usize,
    width: usize,
}

impl IndentWriter {
    pub fn new(amount: usize) -> Self {
        Self {
            buffer: String::new(),
            level: 0,
            amount,
            width: 88,
        }
    }

    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    pub fn with_indent<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.level += 1;
        f(self);
        self.level -= 1;
    }

    pub fn writeln(&mut self, text: &str) -> &mut Self {
        self.writeln_with(0, text, "");
        self
    }

    fn writeln_with(&mut self, extra_levels: usize, text: &str, sep: &str) -> &mut Self {
        use std::fmt::Write;

        let lines: Vec<&str> = text.lines().collect();
        let common_indent = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        let indent_str = " ".repeat(self.amount * (self.level + extra_levels));

        let line_sep = format!("{}\n", sep);
        let result = lines
            .iter()
            .map(|line| line.trim_end())
            .map(|line| {
                if line.trim_start().is_empty() {
                    line.to_string()
                } else {
                    format!("{}{}", indent_str, &line[common_indent..])
                }
            })
            .collect::<Vec<_>>()
            .join(line_sep.as_str());

        for line in result.lines() {
            writeln!(&mut self.buffer, "{}", line).unwrap();
        }
        self
    }

    pub fn fold(
        &mut self,
        extra_levels: usize,
        items: &[String],
        prefix: &str,
        sep: &str,
        suffix: &str,
    ) -> &mut Self {
        let current_indent = self.amount * (self.level + 1);
        let available_width = self.width.saturating_sub(current_indent);

        let single_line = format!(
            "{}{}{}",
            prefix,
            items.join(format!("{} ", sep).as_str()),
            suffix
        );
        if single_line.len() <= available_width && single_line.lines().count() == 1 {
            self.writeln_with(extra_levels, &single_line, "")
        } else {
            self.writeln(prefix);
            self.writeln_with(1 + extra_levels, &items.join("\n"), sep);
            self.writeln(suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_indentation() {
        let mut iw = IndentWriter::new(4);
        iw.writeln("line1");
        iw.with_indent(|p| {
            p.writeln("line2");
        });
        iw.writeln("line3");

        let expected = "line1\n    line2\nline3\n";
        assert_eq!(iw.take(), expected);
    }

    #[test]
    fn test_multiline_normalization() {
        let mut iw = IndentWriter::new(2);
        // Multiline string with its own indentation
        let multiline = "
            def func():
                pass
        ";
        // This should strip the common 12 spaces and add 2 (from indent)
        iw.with_indent(|p| {
            p.writeln(multiline);
        });

        let output = iw.take();
        assert!(output.contains("  def func():\n"));
        assert!(output.contains("    pass\n"));
    }

    #[test]
    fn test_fold_horizontal() {
        let mut iw = IndentWriter::new(4);
        let items = ["a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        // Correct order: extra_levels, items, prefix, lbrack, sep, rbrack
        iw.fold(0, &items, "vec:[", ",", "]");

        assert_eq!(iw.take(), "vec:[a, b, c]\n");
    }

    #[test]
    fn test_fold_vertical() {
        let mut iw = IndentWriter::new(4);
        iw.width = 15; // Force vertical break
        let items = vec!["item1".to_string(), "item2".to_string()];

        // Correct order: extra_levels, items, prefix, lbrack, sep, rbrack
        iw.fold(0, &items, "data:{", ",", "}");

        let output = iw.take();
        // data:{ ends the first line
        //     item1, (indented + sep)
        //     item2 (indented, no sep)
        // } (closing bracket)
        let expected = "data:{\n    item1,\n    item2\n}\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_take_is_destructive() {
        let mut iw = IndentWriter::new(4);
        iw.writeln("content");

        let first = iw.take();
        let second = iw.take();

        assert_eq!(first, "content\n");
        assert_eq!(second, "");
    }

    #[test]
    fn test_nested_folds() {
        let mut iw = IndentWriter::new(2);
        iw.width = 20;

        iw.with_indent(|p| {
            p.fold(
                0,
                &["x".to_string(), "y".to_string()],
                "coords:(",
                ";\
            ",
                ")",
            );
        });

        // coords:(x; y) is 14 chars. + 2 indent = 16. Fits in 20.
        assert_eq!(iw.take(), "  coords:(x; y)\n");
    }
}
