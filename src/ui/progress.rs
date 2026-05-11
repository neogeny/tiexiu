// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::heartbeat::CliHeartbeat;
use tiexiu::HeartbeatRef;

pub struct LoadProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl LoadProgress {
    pub fn new(mp: &indicatif::MultiProgress, msg: &'static str) -> Self {
        let pb = mp.insert(
            0,
            indicatif::ProgressBar::new_spinner().with_style(
                indicatif::ProgressStyle::with_template("{spinner:.cyan} {wide_msg}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            ),
        );
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        pb.set_message(msg);
        Self { pb, hb }
    }

    pub fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    pub fn finish(self) {
        self.pb.finish_with_message("loaded");
    }
}

pub struct FileProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl FileProgress {
    fn new(mp: &indicatif::MultiProgress, name: &str) -> Self {
        let pb = mp.insert(
            0,
            indicatif::ProgressBar::new(0)
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        // "  {prefix:>40.bold} [{wide_bar:.cyan/black}] {pos:>8}/{len:<8} bytes",
                        &("  {prefix:>40.bold} [{wide_bar:.yellow/black}]".to_string()
                            + " {percent:>4}% {duration_precise}  "),
                    )
                    .unwrap()
                    .progress_chars("░▓▒"),
                )
                .with_prefix(name.to_string()),
        );
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        Self { pb, hb }
    }

    pub fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    pub fn set_length(&self, len: usize) {
        self.pb.set_length(len as u64);
    }

    pub fn success(self) {
        self.pb.finish_with_message("done");
    }

    pub fn fail(self, _msg: &str) {
        // self.pb.finish_with_message(msg.to_string());
    }
}

pub struct ProgressUI {
    mp: indicatif::MultiProgress,
    files: indicatif::ProgressBar,
}

impl ProgressUI {
    pub fn new(total: u64) -> Self {
        let mp = indicatif::MultiProgress::new();
        let files = mp.add(indicatif::ProgressBar::new(total)
            .with_style(
                indicatif::ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files",
                )
                .unwrap()
                .progress_chars("⠇⠋ "),
                // .progress_chars("░>-"),
        ));
        Self { mp, files }
    }

    pub fn loading(&self, msg: &'static str) -> LoadProgress {
        LoadProgress::new(&self.mp, msg)
    }

    pub fn add_file(&self, name: &str) -> FileProgress {
        FileProgress::new(&self.mp, name)
    }

    pub fn inc_files(&self) {
        self.files.inc(1);
    }

    pub fn finish(self) {
        self.files.finish_with_message("done");
    }
}
