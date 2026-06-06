// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use console::style;
use std::path::PathBuf;
use tiexiu::Result;
use tiexiu::api::parse_input;
use tiexiu::cfg::Cfg;
use tiexiu::cfg::CfgKey;

use crate::ui::progress::ProgressUI;

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

    if nproc == Some(1) || inputs.len() <= 1 {
        let mut output = String::new();
        let mut errcount = 0;
        for input in &inputs {
            let name = input.file_name().unwrap_or_default().to_string_lossy();
            let file_prog = progress.add_file(&name);

            let text = match std::fs::read_to_string(input) {
                Err(error) => {
                    file_prog.fail(&format!("{:#?}", error));
                    errcount += 1;
                    continue;
                }
                Ok(text) => text,
            };
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
                    file_prog.fail(&format!("{:#?}", err));
                    eprintln!("{}", err);
                }
            }
            progress.inc_files();
        }
        progress.finish();
        eprintln!(
            "{} {} {}",
            style(format!("Parsed {} files", inputs.len()))
                .white()
                .bold(),
            style(format!("{} passed", inputs.len() - errcount))
                .green()
                .bold(),
            if errcount > 0 {
                style(format!("{} errors", errcount)).red().bold()
            } else {
                style("".to_string()).white()
            },
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
        let (tx, rx) = std::sync::mpsc::channel::<(usize, Option<String>, Option<String>)>();

        let inputs = std::sync::Arc::new(inputs);
        let progress = std::sync::Arc::new(progress);
        let parser = std::sync::Arc::new(parser);
        let cfg = std::sync::Arc::new(Cfg::clone(cfg));

        let mut handles = Vec::with_capacity(nworkers);
        for worker_id in 0..nworkers {
            let inputs = std::sync::Arc::clone(&inputs);
            let progress = std::sync::Arc::clone(&progress);
            let parser = std::sync::Arc::clone(&parser);
            let cfg = std::sync::Arc::clone(&cfg);
            let tx = tx.clone();
            handles.push(std::thread::spawn(move || {
                for (file_idx, input) in inputs.iter().enumerate().skip(worker_id).step_by(nworkers)
                {
                    let name = input.file_name().unwrap_or_default().to_string_lossy();
                    let file_prog = progress.add_file(&name);

                    let text = match std::fs::read_to_string(input) {
                        Err(error) => {
                            file_prog.fail(&format!("{:#?}", error));
                            progress.inc_files();
                            tx.send((file_idx, None, Some(error.to_string()))).ok();
                            continue;
                        }
                        Ok(text) => text,
                    };
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
                            progress.inc_files();
                            tx.send((file_idx, Some(this_output), None)).ok();
                        }
                        Err(err) => {
                            file_prog.fail(&format!("{:#?}", err));
                            progress.inc_files();
                            tx.send((file_idx, None, Some(err.to_string()))).ok();
                        }
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let progress = std::sync::Arc::try_unwrap(progress).ok().unwrap();

        drop(tx);
        let mut results: Vec<_> = rx.iter().collect();
        results.sort_by_key(|(id, _, _)| *id);

        let mut output = String::new();
        let mut errcount = 0;
        for (_id, out, err) in results {
            if let Some(out) = out {
                output.push_str(&out);
                output.push('\n');
            }
            if let Some(err) = err {
                errcount += 1;
                eprintln!("{}", err);
            }
        }

        progress.finish();
        eprintln!(
            "{} {} {}",
            style(format!("Parsed {} files", inputs.len()))
                .white()
                .bold(),
            style(format!("{} passed", inputs.len() - errcount))
                .green()
                .bold(),
            if errcount > 0 {
                style(format!("{} errors", errcount)).red().bold()
            } else {
                style("".to_string()).white()
            },
        );
        if errcount > 0 {
            return Err("Some files could not be parsed".into());
        }
        Ok((output, out_lang))
    }
}
