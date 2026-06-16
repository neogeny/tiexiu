// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use console::Style;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tiexiu::Result;
use tiexiu::api::parse_input;
use tiexiu::cfg::Cfg;
use tiexiu::cfg::CfgKey;

use crate::ui::progress::ProgressUI;
use tiexiu::util::strtools::linecount;

// --- Summary display customization ---------------------------------
// Change these to customize the colors of the end-of-run summary.
// Each row has its own (title_style, value_style) pair.

fn title_style() -> Style {
    Style::new().cyan().dim()
}
fn value_style() -> Style {
    Style::new().white().bright()
}

fn success_title_style() -> Style {
    Style::new().green().dim()
}
fn success_value_style() -> Style {
    Style::new().green().bright()
}

fn failure_title_style() -> Style {
    Style::new().red().dim()
}
fn failure_value_style() -> Style {
    Style::new().red().bright()
}

fn rate_style(pct: f64) -> Style {
    if pct >= 100.0 {
        Style::new().green()
    } else if pct > 0.0 {
        Style::new().yellow()
    } else {
        Style::new().red()
    }
}

struct Summary {
    files_input: usize,
    source_lines: usize,
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
            source_lines: 0,
            success_lines: 0,
            successes: 0,
            failures: 0,
            run_time: Duration::ZERO,
            wall_time: Duration::ZERO,
        }
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    if secs >= 3600 {
        format!("{}:{:02}:{:02}", secs / 3600, (secs % 3600) / 60, secs % 60)
    } else {
        format!("{}:{:02}", secs / 60, secs % 60)
    }
}

fn print_summary(s: &Summary) {
    let t = title_style();
    let v = value_style();

    macro_rules! row {
        ($ts:expr, $vs:expr, $name:expr, $val:expr, $suf:expr) => {
            eprintln!(
                "{:>19} {:>13} {}",
                $ts.apply_to($name),
                $vs.apply_to($val),
                $vs.apply_to($suf)
            )
        };
    }

    let bright_red = Style::new().red().bright();
    eprintln!();
    if s.failures > 0 {
        eprintln!(
            "{}",
            bright_red.apply_to(format!("FAILURES: {}", s.failures))
        );
    }
    eprintln!();

    row!(t, v, "files input", s.files_input, "");
    row!(t, v, "source lines input", s.source_lines, "");
    row!(t, v, "success lines", s.success_lines, "");
    row!(t, v, "sloc", s.success_lines, "");
    row!(
        success_title_style(),
        success_value_style(),
        "successes",
        s.successes,
        ""
    );
    row!(
        failure_title_style(),
        failure_value_style(),
        "failures",
        s.failures,
        ""
    );

    if s.files_input > 0 {
        let pct = s.successes as f64 / s.files_input as f64 * 100.0;
        row!(t, rate_style(pct), "success rate", pct, "%");
    }

    if s.wall_time.as_secs_f64() > 0.0 {
        let sls = s.success_lines as f64 / s.wall_time.as_secs_f64();
        row!(
            t,
            rate_style(sls),
            "sloc/sec",
            format!("{:0.0}", sls),
            "sl/s"
        );
    }
    row!(t, v, "run time", format_duration(s.run_time), "");
    row!(t, v, "wall time", format_duration(s.wall_time), "");
}

enum WorkerResult {
    Success(usize, String, Duration, usize),
    Error(usize, String, Duration),
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

    let out_lang = if model {
        "rs"
    } else if short {
        "rust"
    } else {
        "json"
    };

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
            summary.source_lines += lines;
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
                    file_results.push((name, elapsed, true, None));
                    let this_output = if model {
                        format!("{:#?}", tree).to_string()
                    } else if short {
                        format!("{:#}", tree.fold()).to_string()
                    } else {
                        tree.to_json_string_pretty()
                    };
                    output.push_str(&this_output);
                    output.push('\n');
                }
                Err(err) => {
                    let elapsed = file_start.elapsed();
                    summary.failures += 1;
                    file_results.push((name, elapsed, false, Some(err.to_string())));
                }
            }
            progress.inc_files();
        }
        summary.wall_time = wall_start.elapsed();
        progress.finish();
        for (name, duration, success, _) in &file_results {
            let mark = if *success { "✓" } else { "✗" };
            eprintln!(
                "{} {:<48} {:>5}",
                mark,
                name,
                format!("{:.1}s", duration.as_secs_f64())
            );
        }
        if summary.failures > 0 {
            eprintln!();
            for (_, _, _, err) in &file_results {
                if let Some(err) = err {
                    eprintln!("{}", err);
                }
            }
        }
        print_summary(&summary);
        Ok((output, out_lang))
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

        let mut handles = Vec::with_capacity(nworkers);
        for worker_id in 0..nworkers {
            let inputs = std::sync::Arc::clone(&inputs);
            let parser = std::sync::Arc::clone(&parser);
            let cfg = std::sync::Arc::clone(&cfg);
            let progress = std::sync::Arc::clone(&progress);
            let tx = tx.clone();
            handles.push(std::thread::spawn(move || {
                for (file_idx, input) in inputs.iter().enumerate().skip(worker_id).step_by(nworkers)
                {
                    let text = match std::fs::read_to_string(input) {
                        Err(error) => {
                            tx.send(WorkerResult::Error(
                                file_idx,
                                error.to_string(),
                                Duration::ZERO,
                            ))
                            .ok();
                            continue;
                        }
                        Ok(text) => text,
                    };

                    // Create per-file bar lazily, only when a task actually runs.
                    let (fp, heartbeat) = {
                        let prog = progress.lock().unwrap();
                        let name = input.file_name().unwrap_or_default().to_string_lossy();
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

                    match parse_input(&parser, &text, &file_cfg) {
                        Ok(tree) => {
                            let elapsed = file_start.elapsed();
                            {
                                let prog = progress.lock().unwrap();
                                fp.success();
                                prog.inc_files();
                            }
                            let sloc = linecount(&text);
                            let this_output = if model {
                                format!("{:#?}", tree).to_string()
                            } else if short {
                                format!("{:#}", tree.fold()).to_string()
                            } else {
                                tree.to_json_string_pretty()
                            };
                            tx.send(WorkerResult::Success(file_idx, this_output, elapsed, sloc))
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
                            tx.send(WorkerResult::Error(file_idx, err.to_string(), elapsed))
                                .ok();
                        }
                    }
                }
            }));
        }

        drop(tx);

        let mut results: Vec<(usize, Option<String>, Option<String>, Duration)> = Vec::new();
        let mut errors: Vec<(String, String)> = Vec::new();
        let mut done = 0;
        while let Ok(event) = rx.recv() {
            match event {
                WorkerResult::Success(idx, out, elapsed, sloc) => {
                    summary.successes += 1;
                    summary.success_lines += sloc;
                    summary.source_lines += sloc;
                    summary.run_time += elapsed;
                    results.push((idx, Some(out), None, elapsed));
                }
                WorkerResult::Error(idx, err, elapsed) => {
                    summary.failures += 1;
                    summary.run_time += elapsed;
                    let name = inputs[idx]
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
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

        results.sort_by_key(|(id, _, _, _)| *id);

        let mut file_results: Vec<(String, Duration, bool)> = Vec::with_capacity(results.len());
        for (idx, out, _, elapsed) in &results {
            let name = inputs[*idx]
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            file_results.push((name, *elapsed, out.is_some()));
        }

        let mut output = String::new();
        for (_id, out, err, _) in &results {
            if err.is_some() {
                continue;
            }
            if let Some(out) = out {
                output.push_str(out);
                output.push('\n');
            }
        }

        summary.wall_time = wall_start.elapsed();
        summary.source_lines = summary.success_lines; // multi-threaded: only track success lines
        progress.lock().unwrap().finish();
        for (name, duration, success) in &file_results {
            let mark = if *success { "✓" } else { "✗" };
            eprintln!(
                "{} {:<48} {:>5}",
                mark,
                name,
                format!("{:.1}s", duration.as_secs_f64())
            );
        }
        if !errors.is_empty() {
            eprintln!();
            for (_, err) in &errors {
                eprintln!("{}", err);
            }
        }
        print_summary(&summary);
        Ok((output, out_lang))
    }
}
