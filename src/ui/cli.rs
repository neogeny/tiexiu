// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::Error;
use crate::json::boot::boot_grammar;
use crate::json::tatsu::TatSuModel;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use owo_colors;
use std::path::PathBuf;

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold())
        .usage(AnsiColor::Yellow.on_default().bold())
        .literal(AnsiColor::Green.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    styles = cli_styles()
)]
/// TieXiu: A high-performance PEG engine for TatSu grammars.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Control when to use color in output.
    #[arg(
        long,
        value_enum,
        default_value_t = clap::ColorChoice::Auto,
        global = true
    )]
    pub color: clap::ColorChoice,
}

#[derive(Subcommand)]
pub enum Commands {
    /// The internal boot grammar
    Boot {
        /// Print the boot grammar in JSON format
        #[arg(short, long, default_value_t = true)]
        json: bool,

        /// Pretty-print the boot grammar
        #[arg(short, long)]
        pretty: bool,
    },
    /// Execute a grammar against one or more input files.
    Run {
        /// Path to the compiled TatSu JSON grammar.
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

pub fn cli() -> Result<(), Error> {
    let cli = Cli::parse();

    // Determine if we should use color
    let use_color = match cli.color {
        clap::ColorChoice::Always => true,
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Auto => {
            // Check if stdout is a TTY (terminal)
            // Note: is_terminal() requires standard library 1.70+
            std::io::IsTerminal::is_terminal(&std::io::stdout())
        }
    };

    // Set global override for owo-colors
    if !use_color {
        owo_colors::set_override(false);
    }

    match cli.command {
        Commands::Boot { pretty, json } => {
            let bootg = boot_grammar()?;
            if pretty {
                pygmentize(&bootg.to_string(), "ebnf", use_color);
                return Ok(());
            }
            if json {
                let model: TatSuModel = bootg.clone().try_into()?;
                let json_str = serde_json::to_string_pretty(&model)?;

                pygmentize(&json_str, "json", use_color);
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

    Ok(())
}

pub fn pygmentize(content: &str, extension: &str, use_color: bool) {
    if !use_color {
        println!("{}", content);
        return;
    }

    use syntect::easy::HighlightLines;
    use syntect::highlighting::ThemeSet;
    use syntect::parsing::SyntaxSet;
    use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension(extension)
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    for line in LinesWithEndings::from(content) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        // false = do not force background colors
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
    // Reset terminal to default state
    print!("\x1b[0m");
    if !content.ends_with('\n') {
        println!();
    }
}
