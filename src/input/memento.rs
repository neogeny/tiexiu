use console::style;
use std::fmt;

pub struct Memento {
    /// The specific error message (e.g., "expected semicolon")
    pub msg: Box<str>,
    /// Absolute line number and content of captured lines
    pub window: Box<[(usize, Box<str>)]>,
    /// (Absolute Line, Absolute Column) - 1-indexed
    pub abs_start: (usize, usize),
    /// (Absolute Line, Absolute Column) - 1-indexed
    pub abs_mark: (usize, usize),
    /// Indices within the `window` slice for annotation
    pub rel_start_idx: usize,
    pub rel_mark_idx: usize,
}

impl Memento {
    pub fn new(text: &str, start: usize, mark: usize, msg: &str) -> Self {
        // 1. Get absolute positions (1-indexed)
        let abs_start = Self::pos_at(text, start);
        let abs_mark = Self::pos_at(text, mark);

        // 2. Define window boundaries (4 lines before the mark line)
        let mark_line_idx = abs_mark.0.saturating_sub(1);
        let start_line_idx = mark_line_idx.saturating_sub(4);

        // 3. Capture lines into the boxed window
        let window_vec: Vec<(usize, Box<str>)> = text
            .lines()
            .enumerate()
            .skip(start_line_idx)
            .take(mark_line_idx - start_line_idx + 1)
            .map(|(i, line)| (i + 1, Box::from(line)))
            .collect();

        // 4. Map absolute positions to window-relative indices
        let rel_start_idx = abs_start.0.saturating_sub(start_line_idx + 1);
        let rel_mark_idx = abs_mark.0.saturating_sub(start_line_idx + 1);

        Self {
            msg: Box::from(msg),
            window: window_vec.into_boxed_slice(),
            abs_start,
            abs_mark,
            rel_start_idx,
            rel_mark_idx,
        }
    }

    /// Platform-independent 1-indexed position retrieval
    fn pos_at(text: &str, mut mark: usize) -> (usize, usize) {
        mark = mark.min(text.len());
        let head = &text[0..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(0, |l| l.chars().count());
        (line, col)
    }

    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_label = style("error").red().bold();
        let blue_pipe = style("|").blue().bold();
        let arrow = style("-->").blue().bold();

        writeln!(f)?;
        // 1. Main Error Line: error: <msg>
        writeln!(f, "\n{}: {}", err_label, style(&self.msg).bold())?;

        let filepath = "input";
        // 2. Location Line: --> input:line:col
        writeln!(
            f,
            "  {} {}:{}:{}",
            arrow, filepath, self.abs_start.0, self.abs_start.1
        )?;

        // 3. Leading Pipe
        writeln!(f, "   {}", blue_pipe)?;

        // 4. Code Window
        for (idx, (num, content)) in self.window.iter().enumerate() {
            writeln!(
                f,
                "{:>2} {} {}",
                style(num).blue().bold(),
                blue_pipe,
                content
            )?;

            // If this is the 'mark' line, render the caret and the specific message
            if idx == self.rel_mark_idx {
                let padding = " ".repeat(self.abs_mark.1);
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

        Ok(())
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
