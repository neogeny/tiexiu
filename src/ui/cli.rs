// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::api::{boot_grammar_json, boot_grammar_pretty, compile, load, parse_input};
use crate::cfg::CfgA;
pub use crate::json::exp_json::*;
pub use crate::peg::pretty::*;
pub use crate::tools::rails::*;
use crate::{Cfg, Result, boot_grammar, config};
use clap;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold())
        .usage(AnsiColor::Yellow.on_default().bold())
        .literal(AnsiColor::Green.on_default().bold())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Output the grammar as a minified JSON object
    Json,
    /// Output the grammar using the pretty-printed EBNF format
    Pretty,
    /// Output a Railroad Diagram (using APL-style characters)
    Railroads,
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

    /// Output to a file instead of stdout
    #[arg(short, long, global = true, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Control when to use color in output.
    #[arg(
        long,
        value_enum,
        default_value_t = clap::ColorChoice::Auto,
        global = true
    )]
    pub color: clap::ColorChoice,

    /// Display a detailed trace of the parsing process.
    #[arg(long, default_value_t = false, global = true)]
    pub trace: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// The internal boot grammar
    Boot {
        /// Print the boot grammar in JSON format
        #[arg(short, long, default_value_t = true)]
        json: bool,

        /// Print the Rust code for the boot grammar model
        #[arg(short, long)]
        model: bool,

        /// Pretty-print the boot grammar
        #[arg(short, long)]
        pretty: bool,

        /// Print a railroad diagram
        #[arg(short, long, group = "format")]
        railroads: bool,
    },
    /// Execute a grammar against one or more input files.
    Run {
        /// Path to the compiled TatSu JSON grammar.
        #[arg(required = true)]
        grammar: PathBuf,

        /// The files to be parsed.
        #[arg(required = true)]
        inputs: Vec<PathBuf>,
    },

    /// Grammar transformations
    Grammar {
        /// Path to the compiled grammar (.ebnf or .json)
        #[arg(required = true)]
        grammar: PathBuf,

        /// Print the grammar in JSON format
        #[arg(short, long, group = "format")]
        json: bool,

        /// Print the Rust code for the boot grammar model
        #[arg(short, long)]
        model: bool,

        /// Pretty-print the grammar (EBNF)
        #[arg(short, long, group = "format")]
        pretty: bool,

        /// Print a railroad diagram
        #[arg(short, long, group = "format")]
        railroads: bool,
    },
}

pub fn cli() -> Result<()> {
    let cli = Cli::parse();
    let use_color = configure_color(cli.color);

    let mut cfg = config(&[]);
    if cli.trace {
        cfg = cfg.add(Cfg::Trace);
    }
    let cfga: &CfgA = &cfg;

    let (content, lang) = match cli.command {
        Commands::Boot {
            model,
            pretty,
            railroads,
            ..
        } => {
            if pretty {
                (boot_grammar_pretty(cfga)?, "ebnf")
            } else if model {
                (format!("{:#?}", boot_grammar()?), "rs")
            } else if railroads {
                (boot_grammar()?.railroads(), "apl")
            } else {
                // Since json is default_value_t = true, this is the fallthrough
                (boot_grammar_json(cfga)?, "json")
            }
        }
        Commands::Run {
            grammar, inputs, ..
        } => {
            let parser = load_grammar_from_path(&grammar, cfga)?;
            let mut output = String::new();
            for input in inputs {
                let text = std::fs::read_to_string(&input)?;
                let tree = parse_input(&parser, &text, cfga)?;
                output.push_str(format!("{}", tree.into_node_tree()).as_str());
                output.push('\n');
            }
            (output, "text")
        }
        Commands::Grammar {
            grammar,
            json,
            model,
            railroads,
            ..
        } => {
            let parser = load_grammar_from_path(&grammar, cfga)?;
            if json {
                (parser.to_json_string()?, "json")
            } else if model {
                (format!("{:#?}", parser), "rs")
            } else if railroads {
                (parser.railroads(), "apl")
            } else {
                (parser.pretty_print(), "ebnf")
            }
        }
    };

    // Dispatch output
    if let Some(path) = cli.output {
        std::fs::write(path, content)?;
    } else {
        pygmentize(&content, lang, use_color);
    }

    Ok(())
}

fn load_grammar_from_path(grammar: &PathBuf, cfga: &CfgA) -> Result<crate::peg::Grammar> {
    let grammar_text = std::fs::read_to_string(grammar)?;
    let parser = if grammar
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        load(&grammar_text, cfga)?
    } else {
        compile(&grammar_text, cfga)?
    };
    Ok(parser)
}

// Optional: Tells clap which global color setting to use for its own help text
fn cli_color_strategy() -> clap::ColorChoice {
    // You can pull from env here if you want clap to be smart before parsing
    clap::ColorChoice::Auto
}

fn configure_color(color: clap::ColorChoice) -> bool {
    match color {
        clap::ColorChoice::Always => true,
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Auto => {
            std::io::IsTerminal::is_terminal(&std::io::stdout())
            // && std::io::IsTerminal::is_terminal(&std::io::stderr())
        }
    }
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
