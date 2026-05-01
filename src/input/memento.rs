// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::engine::state::CallStack;
use crate::types::{Ref, Str};
use console::style;
use std::fmt;

#[derive(Clone)]
pub struct Memento {
    /// The name of the source (e.g., file path)
    pub source: Str,
    /// The specific error message (e.g., "expected semicolon")
    pub msg: Str,
    /// The full input text. Stored as a reference/Arc to avoid copying.
    pub text: Str,
    /// The absolute byte offset of the error
    pub mark: usize,
    /// The start of the relevant span for highlighting
    pub start: usize,
    /// Rule invocations leading to this moment
    pub callstack: CallStack,
}

impl Memento {
    pub fn new(
        source: &str,
        text: &str,
        start: usize,
        mark: usize,
        msg: &str,
        callstack: &CallStack,
    ) -> Self {
        Self {
            source: source.into(),
            msg: Ref::from(msg),
            text: Ref::from(text),
            mark,
            start,
            callstack: callstack.clone(),
        }
    }

    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (line_num, col_num) = Self::pos_at(&self.text, self.mark);

        let err_label = style("error").red().bold();
        let blue_pipe = style("|").blue().bold();
        let arrow = style("-->").blue().bold();

        writeln!(f)?;
        writeln!(f, "{}: {}", err_label, style(&self.msg).bold())?;
        writeln!(f, "  {} {}:{}:{}", arrow, self.source, line_num, col_num)?;
        writeln!(f, "   {}", blue_pipe)?;

        // Windowing logic: find line boundaries without pre-collecting
        let lines: Vec<&str> = self.text.lines().collect();
        let mark_line_idx = line_num.saturating_sub(1);
        let start_line_idx = mark_line_idx.saturating_sub(4);

        for i in start_line_idx..=mark_line_idx {
            if let Some(content) = lines.get(i) {
                let current_line_num = i + 1;
                writeln!(
                    f,
                    "{:>2} {} {}",
                    style(current_line_num).blue().bold(),
                    blue_pipe,
                    content
                )?;

                if current_line_num == line_num {
                    let padding = " ".repeat(col_num.saturating_sub(1));
                    writeln!(
                        f,
                        "   {} {}{} {}",
                        blue_pipe,
                        padding,
                        style("^").red().bold(),
                        style(&self.msg).red()
                    )?;
                }
            }
        }

        writeln!(f)?;
        for call in self.callstack.iter() {
            writeln!(f, "{}", style(call).white().bold())?;
        }

        Ok(())
    }

    /// Calculates 1-indexed (line, column) from a byte offset
    fn pos_at(text: &str, mark: usize) -> (usize, usize) {
        let mark = mark.min(text.len());
        let head = &text[..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(1, |l| l.chars().count() + 1);
        (line, col)
    }
}

impl fmt::Display for Memento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f)
    }
}

impl fmt::Debug for Memento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f)
    }
}
