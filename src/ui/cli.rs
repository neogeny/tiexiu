// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
/// TieXiu: A high-performance PEG engine for TatSu grammars.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    /// Execute a grammar against one or more input files.
    Run {
        /// Path to the compiled TatSu JSON grammar.
        // #[arg(short, long)]
        #[arg(required = true)]
        grammar: String,

        /// The files to be parsed.
        #[arg(required = true)]
        inputs: Vec<String>,

        /// Display a detailed trace of the parsing process.
        #[arg(short, long)]
        trace: bool,
    },
}
