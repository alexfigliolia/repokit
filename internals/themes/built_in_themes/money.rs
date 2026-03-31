use colored::Color;

use crate::themes::theme_colors::ThemeColors;

pub const MONEY: ThemeColors = ThemeColors {
    prefixColor: Color::TrueColor {
        r: 26,
        g: 227,
        b: 133,
    },
    commandColor: Color::TrueColor {
        r: 82,
        g: 234,
        b: 74,
    },
    subcommandColor: Color::TrueColor {
        r: 51,
        g: 241,
        b: 162,
    },
    argColor: Color::TrueColor {
        r: 124,
        g: 244,
        b: 102,
    },
    descriptionColor: Color::TrueColor {
        r: 126,
        g: 168,
        b: 140,
    },
    errorPrefixColor: Color::TrueColor {
        r: 220,
        g: 36,
        b: 100,
    },
    highlightColor: Color::TrueColor {
        r: 25,
        g: 206,
        b: 91,
    },
};
