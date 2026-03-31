use colored::Color;

use crate::themes::theme_colors::ThemeColors;

pub const SEEING_RED: ThemeColors = ThemeColors {
    prefixColor: Color::TrueColor {
        r: 220,
        g: 36,
        b: 91,
    },
    commandColor: Color::TrueColor {
        r: 220,
        g: 36,
        b: 36,
    },
    subcommandColor: Color::TrueColor {
        r: 220,
        g: 131,
        b: 36,
    },
    argColor: Color::TrueColor {
        r: 220,
        g: 205,
        b: 36,
    },
    descriptionColor: Color::TrueColor {
        r: 179,
        g: 100,
        b: 151,
    },
    errorPrefixColor: Color::TrueColor {
        r: 220,
        g: 36,
        b: 39,
    },
    highlightColor: Color::TrueColor {
        r: 237,
        g: 175,
        b: 41,
    },
};
