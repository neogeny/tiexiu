// In your main.rs
let args = Args::parse();

// Map your clap --color argument to bunt's mode
let mode = match args.color {
    ColorPreference::Always => bunt::ColorChoice::Always,
    ColorPreference::Never  => bunt::ColorChoice::Never,
    ColorPreference::Auto   => {
        if owo_colors::supports_colors() {
            bunt::ColorChoice::Auto
        } else {
            bunt::ColorChoice::Never
        }
    }
};

bunt::set_stderr_color_mode(mode);:%y

