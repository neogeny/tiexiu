// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::heartbeat::CliHeartbeat;
use tiexiu::HeartbeatRef;

/// Progress bar for loading global resources (bar style).
pub struct LoadProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl LoadProgress {
    /// Creates a new progress bar for loading, tracking progress via heartbeat.
    pub fn new(mp: &indicatif::MultiProgress, msg: &'static str) -> Self {
        let pb = mp.add(
            indicatif::ProgressBar::new(0)
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        // "  {prefix:>12} {wide_bar:.cyan/blue} {msg}",
                        " {msg} {wide_bar:.cyan/blue}",
                    )
                    .unwrap()
                    .progress_chars("━╸─"),
                )
                .with_prefix("grammar"),
        );
        pb.set_message(msg);
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        Self { pb, hb }
    }

    /// Returns a reference to the underlying heartbeat callback.
    pub fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    /// Sets the expected total number of bytes (from file size).
    pub fn set_length(&self, len: usize) {
        self.pb.set_length(len as u64);
    }

    /// Marks the loading as finished.
    pub fn finish(self) {
        self.pb.finish_and_clear();
    }
}

/// Progress bar for tracking individual file parsing (bar style).
pub struct FileProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl FileProgress {
    fn new(mp: &indicatif::MultiProgress, name: &str) -> Self {
        let pb = mp.add(
            indicatif::ProgressBar::new(0)
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        &(
                            "  {prefix:>40.bold} {wide_bar:.green/black}".to_string() + ""
                            // + " {percent:>4}% {duration_precise}"
                        ),
                    )
                    .unwrap()
                    .progress_chars("━╸─"),
                )
                .with_prefix(name.to_string()),
        );
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        Self { pb, hb }
    }

    /// Returns a reference to the underlying heartbeat callback.
    pub fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    /// Sets the expected total number of bytes.
    pub fn set_length(&self, len: usize) {
        self.pb.set_length(len as u64);
    }

    /// Marks the file as successfully processed.
    pub fn success(self) {
        self.pb.finish_and_clear();
    }

    /// Marks the file as failed (currently a no-op).
    #[allow(dead_code)]
    pub fn fail(self, _msg: &str) {
        self.pb.finish_and_clear();
    }
}

impl Drop for FileProgress {
    fn drop(&mut self) {
        self.pb.finish_and_clear();
    }
}

/// Top-level progress UI with a multi-progress bar for batch processing.
pub struct ProgressUI {
    mp: indicatif::MultiProgress,
    files: indicatif::ProgressBar,
}

impl ProgressUI {
    /// Creates a new progress UI for processing `total` files.
    pub fn new(total: u64) -> Self {
        let mp = indicatif::MultiProgress::new();
        let files = mp.add(
            indicatif::ProgressBar::new(total).with_style(
                indicatif::ProgressStyle::with_template("{wide_bar:.yellow}")
                    .unwrap()
                    .progress_chars("━ "),
            ),
        );
        Self { mp, files }
    }

    /// Starts a spinner progress for a named loading phase.
    pub fn loading(&self, msg: &'static str) -> LoadProgress {
        LoadProgress::new(&self.mp, msg)
    }

    /// Starts a progress bar for processing a named file.
    pub fn add_file(&self, name: &str) -> FileProgress {
        FileProgress::new(&self.mp, name)
    }

    /// Increments the file counter.
    pub fn inc_files(&self) {
        self.files.inc(1);
    }

    /// Marks all processing as finished.
    pub fn finish(&self) {
        self.files.finish_and_clear();
    }
}
