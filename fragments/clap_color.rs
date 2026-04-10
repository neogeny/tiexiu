#[derive(Parser)]
#[command(
    author,
    version,
    about,
    styles = cli_styles(),
    color = cli_color_strategy() // Hook into clap's internal color logic
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        long,
        value_enum,
        default_value_t = clap::ColorChoice::Auto,
        global = true
    )]
    pub color: clap::ColorChoice,
}

// Optional: Tells clap which global color setting to use for its own help text
fn cli_color_strategy() -> clap::ColorChoice {
    // You can pull from env here if you want clap to be smart before parsing
    clap::ColorChoice::Auto
}
