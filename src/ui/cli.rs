// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use tiexiu::api::{
    boot_grammar_pretty, boot_grammar_to_json_string, compile, load_grammar_from_json, parse_input,
};
use tiexiu::cfg::{Cfg, CfgA, Heartbeat, HeartbeatRef};
use tiexiu::peg::pretty::*;
use tiexiu::tools::rails::*;
use tiexiu::{boot_grammar, config, CfgKey, Grammar, Result};

#[derive(Debug)]
struct CliHeartbeat {
    pb: indicatif::ProgressBar,
    last_mark: AtomicUsize,
}

impl CliHeartbeat {
    fn new(pb: indicatif::ProgressBar) -> Self {
        Self {
            pb,
            last_mark: AtomicUsize::new(0),
        }
    }
}

impl Heartbeat for CliHeartbeat {
    fn tick(&self, mark: usize, _total: usize) {
        let prev = self.last_mark.swap(mark, Ordering::Relaxed);
        if mark > prev {
            self.pb.set_position(mark as u64);
        }
    }
}

struct LoadProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl LoadProgress {
    fn new(mp: &indicatif::MultiProgress, msg: &'static str) -> Self {
        let pb = mp.add(
            indicatif::ProgressBar::new_spinner()
                .with_style(
                    indicatif::ProgressStyle::with_template("{spinner:.cyan} {wide_msg}")
                        .unwrap()
                        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
                ),
        );
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        pb.set_message(msg);
        Self { pb, hb }
    }

    fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    fn finish(self) {
        self.pb.finish_with_message("loaded");
    }
}

struct FileProgress {
    pb: indicatif::ProgressBar,
    hb: HeartbeatRef,
}

impl FileProgress {
    fn new(mp: &indicatif::MultiProgress, name: &str) -> Self {
        let pb = mp.add(
            indicatif::ProgressBar::new(0)
                .with_style(
                    indicatif::ProgressStyle::with_template(
                        // "  {prefix:>40.bold} [{wide_bar:.cyan/black}] {pos:>8}/{len:<8} bytes",
                        "  {prefix:>40.bold} [{wide_bar:.cyan/black}] {percent:>4}% ",
                    )
                    .unwrap()
                    .progress_chars("▓▒░"),
                )
                .with_prefix(name.to_string()),
        );
        let hb = std::sync::Arc::new(CliHeartbeat::new(pb.clone()));
        Self { pb, hb }
    }

    fn heartbeat(&self) -> &HeartbeatRef {
        &self.hb
    }

    fn set_length(&self, len: usize) {
        self.pb.set_length(len as u64);
    }

    fn success(self) {
        self.pb.finish_with_message("done");
    }

    fn fail(self) {
        self.pb.finish_with_message("failed");
    }
}

struct ProgressUI {
    mp: indicatif::MultiProgress,
    files: indicatif::ProgressBar,
}

impl ProgressUI {
    fn new(total: u64) -> Self {
        let mp = indicatif::MultiProgress::new();
        let files = mp.add(indicatif::ProgressBar::new(total)
            .with_style(
                indicatif::ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files",
                )
                .unwrap()
                .progress_chars("#>-"),
        ));
        Self { mp, files }
    }

    fn loading(&self, msg: &'static str) -> LoadProgress {
        LoadProgress::new(&self.mp, msg)
    }

    fn add_file(&self, name: &str) -> FileProgress {
        FileProgress::new(&self.mp, name)
    }

    fn inc_files(&self) {
        self.files.inc(1);
    }

    fn finish(self) {
        self.files.finish_with_message("done");
    }
}

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

        /// Print the output tree in JSON format
        #[arg(short, long, group = "format")]
        json: bool,

        /// Print the Rust code for the tree
        #[arg(short, long)]
        model: bool,

        /// Print the Tree in "short" notation
        #[arg(short, long)]
        short: bool,
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

pub fn cli(out: &mut std::io::StdoutLock) -> Result<()> {
    use std::io::Write;
    let cli = Cli::parse();
    let use_color = configure_color(cli.color);

    console::set_colors_enabled(use_color);
    console::set_colors_enabled_stderr(use_color);
    console::set_true_colors_enabled(use_color);
    console::set_true_colors_enabled_stderr(use_color);

    let mut cfg = config(&[]);
    if cli.trace {
        cfg = cfg.add(CfgKey::Trace);
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
                (boot_grammar_to_json_string(cfga)?, "json")
            }
        }
        Commands::Run {
            grammar,
            inputs,
            model,
            short,
            ..
        } => {
            let progress = ProgressUI::new(inputs.len() as u64);
            let parser = load_grammar_from_path(&grammar, &progress, &cfg)?;

            let mut format = "rust";
            let mut output = String::new();

            for input in &inputs {
                let name = input.file_name().unwrap_or_default().to_string_lossy();
                let file_prog = progress.add_file(&name);

                let text = std::fs::read_to_string(input)?;
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
                            format = "json";
                            tree.to_json_string_pretty()
                        };

                        output.push_str(&this_output);
                        output.push('\n');
                    }
                    Err(err) => {
                        file_prog.fail();
                        eprintln!("{:#?}", err)
                    }
                }
                progress.inc_files();
            }
            progress.finish();
            (output, format)
        }
        Commands::Grammar {
            grammar,
            json,
            model,
            railroads,
            ..
        } => {
            let grammar_text = std::fs::read_to_string(&grammar)?;
            let parser = if grammar
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                load_grammar_from_json(&grammar_text, &cfg)?
            } else {
                compile(&grammar_text, &cfg)?
            };
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
        let result = pygmentize(&content, lang, use_color)?;
        write!(out, "{}", result)?;
        out.write_all(&[])?;
    }

    Ok(())
}

fn load_grammar_from_path(grammar: &PathBuf, progress: &ProgressUI, cfga: &CfgA) -> Result<Grammar> {
    let loader = progress.loading("loading grammar");
    let load_cfg = cfga
        .iter()
        .cloned()
        .chain(std::iter::once(CfgKey::Heartbeat(loader.heartbeat().clone())))
        .collect::<Cfg>();

    let grammar_text = std::fs::read_to_string(grammar)?;
    let parser = if grammar
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        load_grammar_from_json(&grammar_text, &load_cfg)?
    } else {
        compile(&grammar_text, &load_cfg)?
    };
    loader.finish();
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
                && std::io::IsTerminal::is_terminal(&std::io::stderr())
        }
    }
}

pub fn pygmentize(content: &str, extension: &str, use_color: bool) -> Result<String> {
    let mut out = String::new();

    if !use_color {
        writeln!(out, "{}", content)?;
        return Ok(out);
    }

    use syntect::easy::HighlightLines;
    use syntect::highlighting::ThemeSet;
    use syntect::parsing::SyntaxSet;
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension(extension)
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    for line in LinesWithEndings::from(content) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        write!(out, "{}", escaped)?;
    }
    write!(out, "\x1b[0m")?;
    if !content.ends_with('\n') {
        writeln!(out)?;
    };
    Ok(out)
}
