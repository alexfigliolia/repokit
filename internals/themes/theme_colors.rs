use colored::Color;

use crate::themes::theme_inputs::ThemeInputColors;

#[derive(Clone)]
pub struct ThemeColors {
    pub prefixColor: Color,
    pub commandColor: Color,
    pub subcommandColor: Color,
    pub argColor: Color,
    pub descriptionColor: Color,
    pub errorPrefixColor: Color,
    pub highlightColor: Color,
}

impl ThemeColors {
    pub fn from_options(input: ThemeInputColors) -> ThemeColors {
        ThemeColors {
            prefixColor: input.prefixColor.unwrap_or(Color::BrightMagenta),
            commandColor: input.commandColor.unwrap_or(Color::BrightBlue),
            subcommandColor: input.subcommandColor.unwrap_or(Color::TrueColor {
                r: 175,
                g: 247,
                b: 7,
            }),
            argColor: input.argColor.unwrap_or(Color::Green),
            descriptionColor: input.descriptionColor.unwrap_or(Color::TrueColor {
                r: 128,
                g: 128,
                b: 128,
            }),
            errorPrefixColor: input.errorPrefixColor.unwrap_or(Color::Red),
            highlightColor: input.highlightColor.unwrap_or(Color::BrightBlue),
        }
    }
}
