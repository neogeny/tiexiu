// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::ui::progress::ProgressUI;
use console::{Style, Term};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tiexiu::Result;
use tiexiu::Tree;
use tiexiu::api::parse_input;
use tiexiu::cfg::Cfg;
use tiexiu::cfg::CfgKey;
use tiexiu::util::finally;
use tiexiu::util::strtools::{LineCount, countlines, linecount};

struct RunUI {
    title: Style,
    value: Style,
    success_title: Style,
    success_value: Style,
    failure_title: Style,
    failure_value: Style,
    bright_red: Style,
    out_lang: &'static str,
    stack_size: usize,
}

impl RunUI {
    fn new(model: bool, short: bool) -> Self {
        Self {
            title: Style::new().cyan().dim(),
            value: Style::new().white().bright(),
            success_title: Style::new().green().dim(),
            success_value: Style::new().green().bright(),
            failure_title: Style::new().red().dim(),
            failure_value: Style::new().red().bright(),
            bright_red: Style::new().red().bright(),
            out_lang: if model {
                "rs"
            } else if short {
                "rust"
            } else {
                "json"
            },
            stack_size: 4 * 1024 * 1024,
        }
    }

    fn rate_style(&self, pct: f64) -> Style {
        if pct >= 100.0 {
            Style::new().green()
        } else if pct > 0.0 {
            Style::new().yellow()
        } else {
            Style::new().red()
        }
    }

    fn row(&self, ts: &Style, vs: &Style, name: &str, val: usize, suf: &str) {
        eprintln!(
            "{:>19} {:>13} {}",
            ts.apply_to(name),
            vs.apply_to(val),
            vs.apply_to(suf)
        )
    }

    fn row_str(&self, ts: &Style, vs: &Style, name: &str, val: &str, suf: &str) {
        eprintln!(
            "{:>19} {:>13} {}",
            ts.apply_to(name),
            vs.apply_to(val),
            vs.apply_to(suf)
        )
    }

    fn format_duration(&self, d: Duration) -> String {
        let secs = d.as_secs();
        if secs >= 3600 {
            format!("{}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
        } else {
            format!("{}:{:02}", secs / 60, secs % 60)
        }
    }

    fn print_file_result(&self, pb: &ProgressUI, name: &str, duration: Duration, success: bool) {
        let s = Style::new();
        let mark = if success {
            s.green().apply_to("✓")
        } else {
            s.red().apply_to("✗")
        };
        pb.files.println(format!(
            "{} {:<48} {:>5}",
            mark,
            name,
            format!("{:.1}s", duration.as_secs_f64())
        ));
    }

    fn print_summary(&self, s: &Summary) {
        eprintln!();
        if s.failures > 0 {
            eprintln!(
                "{}",
                self.bright_red
                    .apply_to(format!("FAILURES: {}", s.failures))
            );
        }
        eprintln!();

        self.row(&self.title, &self.value, "files input", s.files_input, "");
        self.row(
            &self.title,
            &self.value,
            "source lines input",
            s.total_lines,
            "",
        );
        self.row(
            &self.title,
            &self.value,
            "success lines",
            s.success_lines,
            "",
        );
        self.row(&self.title, &self.value, "sloc", s.success_lines, "");
        self.row(
            &self.success_title,
            &self.success_value,
            "successes",
            s.successes,
            "",
        );
        self.row(
            &self.failure_title,
            &self.failure_value,
            "failures",
            s.failures,
            "",
        );

        if s.files_input > 0 {
            let pct = s.successes as f64 / s.files_input as f64 * 100.0;
            self.row_str(
                &self.title,
                &self.rate_style(pct),
                "success rate",
                &format!("{:.0}", pct),
                "%",
            );
        }

        if s.wall_time.as_secs_f64() > 0.0 {
            let sls = s.sloc as f64 / s.wall_time.as_secs_f64();
            self.row_str(
                &self.title,
                &self.rate_style(sls),
                "sloc/sec",
                &format!("{:0.0}", sls),
                "sl/s",
            );
        }
        self.row_str(
            &self.title,
            &self.value,
            "run time",
            &self.format_duration(s.run_time),
            "",
        );
        self.row_str(
            &self.title,
            &self.value,
            "wall time",
            &self.format_duration(s.wall_time),
            "",
        );
    }
}

struct Summary {
    files_input: usize,
    total_lines: usize,
    sloc: usize,
    success_lines: usize,
    successes: usize,
    failures: usize,
    run_time: Duration,
    wall_time: Duration,
}

impl Summary {
    fn new(files_input: usize) -> Self {
        Self {
            files_input,
            total_lines: 0,
            sloc: 0,
            success_lines: 0,
            successes: 0,
            failures: 0,
            run_time: Duration::ZERO,
            wall_time: Duration::ZERO,
        }
    }
}

enum WorkerResult {
    Success(usize, String, String, Duration, LineCount),
    Error(usize, String, String, Duration, LineCount),
}

/// Execute the `Run` subcommand — parse input files with a compiled grammar.
pub fn cmd_run(
    grammar: PathBuf,
    inputs: Vec<PathBuf>,
    model: bool,
    short: bool,
    nproc: Option<usize>,
    cfg: &Cfg,
) -> Result<(String, &'static str)> {
    let progress = ProgressUI::new(inputs.len() as u64);
    let parser = crate::ui::cli::load_grammar_from_path(&grammar, &progress, cfg)?;
    let ui = RunUI::new(model, short);

    let wall_start = Instant::now();
    let mut summary = Summary::new(inputs.len());

    if nproc == Some(1) || inputs.len() <= 1 {
        let mut output = String::new();
        let mut file_results: Vec<(String, Duration, bool, Option<String>)> = Vec::new();
        for input in &inputs {
            let name_os = input.file_name().unwrap_or_default().to_string_lossy();
            let name = name_os.to_string();
            let file_prog = progress.add_file(&name_os);

            let text = match std::fs::read_to_string(input) {
                Err(_) => {
                    drop(file_prog);
                    progress.inc_files();
                    file_results.push((name, Duration::ZERO, false, None));
                    summary.failures += 1;
                    continue;
                }
                Ok(text) => text,
            };
            let lines = linecount(&text);
            summary.total_lines += lines;
            file_prog.set_length(text.len());

            let file_cfg = cfg
                .add(CfgKey::Source(input.as_path().to_string_lossy().into()))
                .add(CfgKey::Heartbeat(file_prog.heartbeat().clone()));

            let file_start = Instant::now();
            match parse_input(&parser, &text, &file_cfg) {
                Ok(tree) => {
                    let elapsed = file_start.elapsed();
                    summary.run_time += elapsed;
                    summary.successes += 1;
                    summary.success_lines += lines;
                    file_prog.success();
                    ui.print_file_result(&progress, &name, elapsed, true);
                    file_results.push((name, elapsed, true, None));
                    let this_output = if model {
                        format!("{:#?}", tree).to_string()
                    } else if short {
                        format!("{:#}", Tree::fold(tree.into())).to_string()
                    } else {
                        tree.to_json_string_pretty()
                    };
                    output.push_str(&this_output);
                    output.push('\n');
                }
                Err(err) => {
                    let elapsed = file_start.elapsed();
                    summary.failures += 1;
                    ui.print_file_result(&progress, &name, elapsed, false);
                    file_results.push((name, elapsed, false, Some(err.to_string())));
                }
            }
            progress.inc_files();
        }
        summary.wall_time = wall_start.elapsed();
        progress.finish();
        for (name, duration, success, _) in &file_results {
            ui.print_file_result(&progress, name, *duration, *success);
        }
        if summary.failures > 0 {
            eprintln!();
            for (_, _, _, err) in &file_results {
                if let Some(err) = err {
                    eprintln!("{}", err);
                }
            }
        }
        ui.print_summary(&summary);
        Ok((output, ui.out_lang))
    } else {
        let max_workers = nproc.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(8)
        });
        let nworkers = max_workers.min(inputs.len());
        let (tx, rx) = std::sync::mpsc::channel::<WorkerResult>();

        // Wrap ProgressUI in a mutex so all MultiProgress operations
        // (insert, finish, drop) are serialized — this prevents a
        // lock-ordering deadlock in indicatif between insert (MP→PB)
        // and drop (PB→MP) when called from different threads.
        let progress = std::sync::Arc::new(std::sync::Mutex::new(progress));
        let inputs = std::sync::Arc::new(inputs);
        let parser = std::sync::Arc::new(parser);
        let cfg = std::sync::Arc::new(Cfg::clone(cfg));

        let term = Term::stderr();
        term.hide_cursor()?;
        eprintln!();

        let mut handles = Vec::with_capacity(nworkers);
        for worker_id in 0..nworkers {
            let inputs = std::sync::Arc::clone(&inputs);
            let parser = std::sync::Arc::clone(&parser);
            let cfg = std::sync::Arc::clone(&cfg);
            let progress = std::sync::Arc::clone(&progress);
            let tx = tx.clone();
            let stack_size = ui.stack_size;
            handles.push(
                std::thread::Builder::new()
                    .stack_size(stack_size)
                    .spawn(move || {
                        for (file_idx, input) in
                            inputs.iter().enumerate().skip(worker_id).step_by(nworkers)
                        {
                            let text = match std::fs::read_to_string(input) {
                                Err(error) => {
                                    tx.send(WorkerResult::Error(
                                        file_idx,
                                        "".to_string(),
                                        error.to_string(),
                                        Duration::ZERO,
                                        LineCount::default(),
                                    ))
                                    .ok();
                                    continue;
                                }
                                Ok(text) => text,
                            };

                            // Create per-file bar lazily, only when a task actually runs.
                            let name = input.file_name().unwrap_or_default().to_string_lossy();
                            let (fp, heartbeat) = {
                                let prog = progress.lock().unwrap();
                                let fp = prog.add_file(&name);
                                if let Ok(meta) = std::fs::metadata(input) {
                                    fp.set_length(meta.len() as usize);
                                }
                                let hb = fp.heartbeat().clone();
                                (fp, hb)
                            };

                            let file_start = Instant::now();
                            let file_cfg = cfg
                                .add(CfgKey::Source(input.as_path().to_string_lossy().into()))
                                .add(CfgKey::Heartbeat(heartbeat));

                            let counts = countlines(&text);
                            match parse_input(&parser, &text, &file_cfg) {
                                Ok(tree) => {
                                    let elapsed = file_start.elapsed();
                                    {
                                        let prog = progress.lock().unwrap();
                                        fp.success();
                                        prog.inc_files();
                                    }
                                    let this_output = if model {
                                        format!("{:#?}", tree).to_string()
                                    } else if short {
                                        format!("{:#}", Tree::fold(tree.into())).to_string()
                                    } else {
                                        tree.to_json_string_pretty()
                                    };
                                    tx.send(WorkerResult::Success(
                                        file_idx,
                                        name.to_string(),
                                        this_output,
                                        elapsed,
                                        counts,
                                    ))
                                    .ok();
                                }
                                Err(err) => {
                                    let elapsed = file_start.elapsed();
                                    {
                                        let prog = progress.lock().unwrap();
                                        // Drop the bar to remove it from MultiProgress.
                                        drop(fp);
                                        prog.inc_files();
                                    }
                                    tx.send(WorkerResult::Error(
                                        file_idx,
                                        name.to_string(),
                                        err.to_string(),
                                        elapsed,
                                        counts,
                                    ))
                                    .ok();
                                }
                            }
                        }
                    })
                    .expect("failed to spawn worker thread"),
            );
        }

        drop(tx);

        let mut results: Vec<(usize, Option<String>, Option<String>, Duration)> = Vec::new();
        let mut errors: Vec<(String, String)> = Vec::new();
        let mut done = 0;
        while let Ok(event) = rx.recv() {
            match event {
                WorkerResult::Success(idx, name, out, elapsed, counts) => {
                    summary.successes += 1;
                    summary.success_lines += counts.totl;
                    summary.total_lines += counts.totl;
                    summary.sloc += counts.code;
                    summary.run_time += elapsed;

                    let pb_locked = progress.lock().unwrap();
                    ui.print_file_result(&pb_locked, &name, elapsed, true);

                    results.push((idx, Some(out), None, elapsed));
                }
                WorkerResult::Error(idx, name, err, elapsed, counts) => {
                    summary.failures += 1;
                    summary.run_time += elapsed;
                    summary.total_lines += counts.totl;
                    summary.sloc += counts.code;

                    let pb_locked = progress.lock().unwrap();
                    ui.print_file_result(&pb_locked, &name, elapsed, true);
                    pb_locked.files.tick();

                    results.push((idx, None, Some(err.clone()), elapsed));
                    errors.push((name, err));
                }
            }
            done += 1;
            if done == inputs.len() {
                break;
            }
        }

        for handle in handles {
            let _ = handle.join();
        }
        summary.wall_time = wall_start.elapsed();
        eprintln!();
        let pb_locked = progress.lock().unwrap();
        pb_locked.finish();

        let mut file_results: Vec<(String, Duration, bool)> = Vec::with_capacity(results.len());
        let mut output = String::new();
        for (idx, out, err, elapsed) in &results {
            let name = inputs[*idx]
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            file_results.push((name, *elapsed, out.is_some()));

            if let Some(error) = err {
                eprintln!("{}", error);
            } else if let Some(out) = out {
                output.push_str(out);
                output.push('\n');
            }
        }
        summary.total_lines = summary.success_lines; // multi-threaded: only track success lines
        finally(|| term.show_cursor().unwrap());
        ui.print_summary(&summary);
        Ok((output, ui.out_lang))
    }
}
