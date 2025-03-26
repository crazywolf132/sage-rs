use clap::builder::Styles;
use owo_colors::{AnsiColors, OwoColorize};

pub fn get_help_styles() -> clap::builder::Styles {
    Styles::styled().header(AnsiColors::Green())
}
