// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::context::CtxI;
use crate::context::state::CallStack;
use crate::types::Str;
use console::style;
use std::rc::Rc;

#[derive(Clone)]
pub struct Memento {
    /// The name of the source (e.g., file path)
    pub input_source: Str,
    /// The specific error (e.g., "expected semicolon")
    pub msg: Str,
    /// The full input text. Stored as a reference/Arc to avoid copying.
    pub text: Rc<str>,
    /// The absolute byte offset of the error
    pub mark: usize,
    /// The start of the relevant span for highlighting
    pub start: usize,
    /// Rule invocations leading to this moment
    pub callstack: CallStack,
}

impl Memento {
    pub fn new(start: usize, ctx: &dyn CtxI, msg: &str) -> Self {
        Self {
            input_source: ctx.cursor().input_source().into(),
            start,
            mark: ctx.mark(),
            msg: msg.into(),
            text: ctx.cursor().as_str().into(),
            callstack: ctx.callstack(),
        }
    }

    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line_num, col_num) = Self::pos_at(&self.text, self.mark);

        let err_label = style("error").red().bold();
        let blue_pipe = style("|").blue().bold();
        let arrow = style("-->").blue().bold();

        let msg = self.msg.to_string();

        writeln!(f)?;
        writeln!(f, "{}: {}", err_label, style(&msg).bold())?;
        writeln!(
            f,
            "  {} {}:{}:{}",
            arrow, self.input_source, line_num, col_num
        )?;
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
                        style(&msg).red()
                    )?;
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            writeln!(f)?;
            for call in self.callstack.iter() {
                writeln!(f, " {} {}", style("→").red(), style(call).black().bright(),)?;
            }
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
