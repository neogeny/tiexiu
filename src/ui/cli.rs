// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::json::tatsu::TatSuModel;
use crate::peg::Grammar;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
/// TieXiu: A high-performance PEG engine for TatSu grammars.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// The internal boot grammar
    Boot {
        /// Pretty print the boot grammar
        #[arg(short, long, default_value_t = true)]
        print: bool,

        /// Print the boot grammar in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Execute a grammar against one or more input files.
    Run {
        /// Path to the compiled TatSu JSON grammar.
        // #[arg(short, long)]
        #[arg(required = true)]
        grammar: PathBuf,

        /// The files to be parsed.
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Display a detailed trace of the parsing process.
        #[arg(short, long)]
        trace: bool,
    },
}

pub fn cli() {
    use crate::ui::cli::{Cli, Commands};
    use clap::Parser;
    let cli = Cli::parse();

    match cli.command {
        Commands::Boot { print, json } => {
            let bootg = Grammar::boot().unwrap();
            if json {
                let model: TatSuModel = bootg.clone().try_into().unwrap();
                let json: serde_json::Value = serde_json::to_value(model).unwrap();
                println!("{:#}", json);
                return;
            }
            if print {
                println!("{}", bootg);
            }
        }
        Commands::Run {
            grammar, inputs, ..
        } => {
            println!(
                "Ready to parse with grammar {}  {}",
                grammar.as_path().to_str().unwrap(),
                inputs
                    .iter()
                    .map(|p| p.as_path().to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }
}
