use colored::Color;

use crate::themes::theme_colors::ThemeColors;

pub const SEEING_RED: ThemeColors = ThemeColors {
    prefix_color: Color::TrueColor {
        r: 220,
        g: 36,
        b: 91,
    },
    command_color: Color::TrueColor {
        r: 220,
        g: 36,
        b: 36,
    },
    subcommand_color: Color::TrueColor {
        r: 220,
        g: 131,
        b: 36,
    },
    arg_color: Color::TrueColor {
        r: 220,
        g: 205,
        b: 36,
    },
    description_color: Color::TrueColor {
        r: 179,
        g: 100,
        b: 151,
    },
    error_prefix_color: Color::TrueColor {
        r: 220,
        g: 36,
        b: 39,
    },
    highlight_color: Color::TrueColor {
        r: 237,
        g: 175,
        b: 41,
    },
};
