// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::Ctx;
use crate::context::state::CallStack;
use crate::types::Str;
use console::style;

/// A parse memento for structured error reporting.
#[derive(Clone)]
pub struct Memento {
    /// The name of the source (e.g., file path)
    pub input_source: Str,
    /// The specific error (e.g., "expected semicolon")
    pub msg: Str,
    /// The full input text. Stored as a reference/Arc to avoid copying.
    pub text: Str,
    /// The start of the relevant span for highlighting
    pub start: usize,
    /// The absolute byte offset of the error
    pub mark: usize,
    /// Rule invocations leading to this moment
    pub callstack: CallStack,
    /// (line, column) position.
    pub pos: (usize, usize),
    /// Lookahead text at the error location.
    pub la: Str,
}

impl Memento {
    /// Create a new `Memento` from a context and message.
    pub fn new(start: usize, ctx: &dyn Ctx, msg: &str) -> Self {
        Self {
            input_source: ctx.cursor().input_source().into(),
            start,
            mark: ctx.mark(),
            msg: msg.into(),
            text: ctx.cursor().as_str().into(),
            callstack: ctx.callstack(),
            pos: ctx.cursor().pos(),
            la: ctx.cursor().lookahead(start).into(),
        }
    }

    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line_num, col_num) = Self::pos_at(&self.text, self.mark);

        let err_label = style("error:").red().bold();
        let blue_pipe = style("│").blue().bold();
        let arrow = style("─→").blue().bold();

        let msg = self.msg.to_string();
        let err_msg = format!("{} {}", err_label, style(&msg).bold());

        writeln!(f)?;
        writeln!(f)?;
        writeln!(f, "{}", err_msg)?;
        writeln!(
            f,
            "  {} {}:{}:{}",
            arrow, self.input_source, line_num, col_num
        )?;

        // Windowing logic: find line boundaries without pre-collecting
        let lines: Vec<&str> = self.text.lines().collect();
        let mark_line_idx = line_num.saturating_sub(1);
        let start_line_idx = mark_line_idx.saturating_sub(4);

        writeln!(f, "{:>4} {}", "", blue_pipe)?;
        for i in start_line_idx..=mark_line_idx {
            if let Some(content) = lines.get(i) {
                let current_line_num = i + 1;
                writeln!(
                    f,
                    "{:>4} {} {}",
                    style(current_line_num).blue().bold(),
                    blue_pipe,
                    content
                )?;
            }
        }
        let padding = " ".repeat(col_num.saturating_sub(1));
        writeln!(
            f,
            "{:>4} {} {}{}",
            "",
            blue_pipe,
            padding,
            style(format!("⌃ {}", msg)).red().bold(),
        )?;

        // #[cfg(debug_assertions)]
        {
            writeln!(f)?;
            for call in self.callstack.iter() {
                writeln!(f, " {} {}", style("→").red(), style(call).black().bright(),)?;
            }
        }
        writeln!(f)?;
        writeln!(f)?;
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

impl std::fmt::Display for Memento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f)
    }
}

impl std::fmt::Debug for Memento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
