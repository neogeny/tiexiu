// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fmt;

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

    pub fn indent<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.level += 1;
        f(self);
        self.level -= 1;
    }

    pub fn writeln(&mut self, text: &str) -> fmt::Result {
        self.printline_with(0, text, "")
    }

    fn printline_with(&mut self, extra_levels: usize, text: &str, sep: &str) -> fmt::Result {
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
            .map(|line| format!("{}{}", indent_str, &line[common_indent..]))
            .collect::<Vec<_>>()
            .join(line_sep.as_str());

        for line in result.lines() {
            writeln!(&mut self.buffer, "{}", line)?;
        }
        Ok(())
    }

    pub fn fold(
        &mut self,
        extra_levels: usize,
        items: &[&str],
        prefix: &str,
        lbrack: &str,
        sep: &str,
        rbrack: &str,
    ) -> fmt::Result {
        let current_indent = self.amount * (self.level + 1);
        let available_width = self.width.saturating_sub(current_indent);

        let single_line = format!(
            "{}{}{}{}",
            prefix,
            lbrack,
            items.join(format!("{} ", sep).as_str()),
            rbrack
        );
        if single_line.len() <= available_width {
            self.printline_with(extra_levels, &single_line, "")
        } else {
            self.writeln(format!("{}{}", prefix, lbrack).as_str())?;
            self.printline_with(1 + extra_levels, &items.join("\n"), sep)?;
            self.writeln(rbrack)
        }
    }
}
