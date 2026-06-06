// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use console::style;
use std::path::PathBuf;
use std::time::Instant;
use tiexiu::Result;
use tiexiu::api::parse_input;
use tiexiu::cfg::Cfg;
use tiexiu::cfg::CfgKey;

use crate::ui::progress::ProgressUI;
use tiexiu::util::strtools::linecount;

enum WorkerResult {
    Success(usize, String, usize),
    Error(usize, String),
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

    let start = Instant::now();
    let mut totl_sloc = 0usize;

    if nproc == Some(1) || inputs.len() <= 1 {
        let mut output = String::new();
        let mut errcount = 0;
        for input in &inputs {
            let name = input.file_name().unwrap_or_default().to_string_lossy();
            let file_prog = progress.add_file(&name);

            let text = match std::fs::read_to_string(input) {
                Err(_) => {
                    drop(file_prog);
                    errcount += 1;
                    progress.inc_files();
                    continue;
                }
                Ok(text) => text,
            };
            totl_sloc += linecount(&text);
            file_prog.set_length(text.len());

            let file_cfg = cfg
                .add(CfgKey::Source(input.as_path().to_string_lossy().into()))
                .add(CfgKey::Heartbeat(file_prog.heartbeat().clone()));

            match parse_input(&parser, &text, &file_cfg) {
                Ok(tree) => {
                    file_prog.success();
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
                    errcount += 1;
                    eprintln!("{}", err);
                }
            }
            progress.inc_files();
        }
        progress.finish();
        let elapsed = start.elapsed();
        let sloc_per_sec = if elapsed.as_secs_f64() > 0.0 {
            totl_sloc as f64 / elapsed.as_secs_f64()
        } else {
            totl_sloc as f64
        };
        eprintln!(
            "{}{}{}{}",
            style(format!("Parsed {} files", inputs.len()))
                .white()
                .bold(),
            style(format!("{} passed", inputs.len() - errcount))
                .green()
                .bold(),
            if errcount > 0 {
                style(format!(" {} errors", errcount)).red().bold()
            } else {
                style("".to_string()).white()
            },
            style(format!("{:.0} sloc/s", sloc_per_sec)).white().dim(),
        );
        if errcount > 0 {
            return Err("Some files could not be parsed".into());
        }
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
                            tx.send(WorkerResult::Error(file_idx, error.to_string()))
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

                    let file_cfg = cfg
                        .add(CfgKey::Source(input.as_path().to_string_lossy().into()))
                        .add(CfgKey::Heartbeat(heartbeat));

                    match parse_input(&parser, &text, &file_cfg) {
                        Ok(tree) => {
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
                            tx.send(WorkerResult::Success(file_idx, this_output, sloc))
                                .ok();
                        }
                        Err(err) => {
                            {
                                let prog = progress.lock().unwrap();
                                // Drop the bar to remove it from MultiProgress.
                                drop(fp);
                                prog.inc_files();
                            }
                            tx.send(WorkerResult::Error(file_idx, err.to_string())).ok();
                        }
                    }
                }
            }));
        }

        drop(tx);

        let mut results: Vec<(usize, Option<String>, Option<String>)> = Vec::new();
        let mut done = 0;
        while let Ok(event) = rx.recv() {
            match event {
                WorkerResult::Success(idx, out, sloc) => {
                    totl_sloc += sloc;
                    results.push((idx, Some(out), None));
                }
                WorkerResult::Error(idx, err) => {
                    results.push((idx, None, Some(err.clone())));
                    eprintln!("{}", err);
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

        results.sort_by_key(|(id, _, _)| *id);

        let mut output = String::new();
        let mut errcount = 0;
        for (_id, out, err) in &results {
            if err.is_some() {
                errcount += 1;
                continue;
            }
            if let Some(out) = out {
                output.push_str(out);
                output.push('\n');
            }
        }

        progress.lock().unwrap().finish();
        let elapsed = start.elapsed();
        let sloc_per_sec = if elapsed.as_secs_f64() > 0.0 {
            totl_sloc as f64 / elapsed.as_secs_f64()
        } else {
            totl_sloc as f64
        };
        eprintln!(
            "{} {}{} {}",
            style(format!("Parsed {} files", inputs.len()))
                .white()
                .bold(),
            style(format!("{} passed", inputs.len() - errcount))
                .green()
                .bold(),
            if errcount > 0 {
                style(format!(" {} errors", errcount)).red().bold()
            } else {
                style("".to_string()).white()
            },
            style(format!("{:.0} sloc/s", sloc_per_sec)).white().dim(),
        );
        if errcount > 0 {
            return Err("Some files could not be parsed".into());
        }
        Ok((output, out_lang))
    }
}
