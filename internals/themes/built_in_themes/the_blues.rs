use colored::Color;

use crate::themes::theme_colors::ThemeColors;

pub const THE_BLUES: ThemeColors = ThemeColors {
    prefix_color: Color::TrueColor {
        r: 36,
        g: 111,
        b: 255,
    },
    command_color: Color::TrueColor {
        r: 52,
        g: 96,
        b: 255,
    },
    subcommand_color: Color::TrueColor {
        r: 0,
        g: 157,
        b: 255,
    },
    arg_color: Color::TrueColor {
        r: 40,
        g: 175,
        b: 253,
    },
    description_color: Color::TrueColor {
        r: 100,
        g: 165,
        b: 179,
    },
    error_prefix_color: Color::TrueColor {
        r: 220,
        g: 36,
        b: 100,
    },
    highlight_color: Color::TrueColor {
        r: 69,
        g: 219,
        b: 229,
    },
};
