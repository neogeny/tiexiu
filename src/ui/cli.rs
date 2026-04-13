// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Result;
use crate::api::{boot_grammar_json, boot_grammar_pretty, compile, load, parse_input};
use clap;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use termcolor;

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
    styles = cli_styles(),
    color = cli_color_strategy() // Hook into clap's internal color logic
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

pub fn cli() -> Result<()> {
    let cli = Cli::parse();

    let use_color = configure_color(cli.color);

    match cli.command {
        Commands::Boot { pretty, json } => {
            if pretty {
                let pretty_str = boot_grammar_pretty()?;
                pygmentize(&pretty_str, "ebnf", use_color);
                return Ok(());
            }
            if json {
                let json_str = boot_grammar_json()?;
                pygmentize(&json_str, "json", use_color);
            }
        }
        Commands::Run {
            grammar, inputs, ..
        } => {
            let grammar_text = std::fs::read_to_string(&grammar)?;
            let parser = if grammar
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                load(&grammar_text)?
            } else {
                compile(&grammar_text)?
            };

            for input in inputs {
                let text = std::fs::read_to_string(&input)?;
                match parse_input(&parser, &text) {
                    Ok(tree) => println!("{}", tree.normalized()),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    Ok(())
}

// Optional: Tells clap which global color setting to use for its own help text
fn cli_color_strategy() -> clap::ColorChoice {
    // You can pull from env here if you want clap to be smart before parsing
    clap::ColorChoice::Auto
}

fn configure_color(color: clap::ColorChoice) -> bool {
    let use_color = match color {
        clap::ColorChoice::Always => true,
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Auto => {
            std::io::IsTerminal::is_terminal(&std::io::stdout())
                && std::io::IsTerminal::is_terminal(&std::io::stderr())
        }
    };
    if !use_color {
    }
    use_color
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
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
    print!("\x1b[0m");
    if !content.ends_with('\n') {
        println!();
    }
}
