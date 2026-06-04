use colored::Color;

use crate::themes::theme_inputs::ThemeInputColors;

#[derive(Clone)]
pub struct ThemeColors {
    pub prefix_color: Color,
    pub command_color: Color,
    pub subcommand_color: Color,
    pub arg_color: Color,
    pub description_color: Color,
    pub error_prefix_color: Color,
    pub highlight_color: Color,
}

impl ThemeColors {
    pub fn from_options(input: ThemeInputColors) -> ThemeColors {
        ThemeColors {
            prefix_color: input.prefix_color.unwrap_or(Color::BrightMagenta),
            command_color: input.command_color.unwrap_or(Color::BrightBlue),
            subcommand_color: input.subcommand_color.unwrap_or(Color::BrightCyan),
            arg_color: input.arg_color.unwrap_or(Color::Green),
            description_color: input.description_color.unwrap_or(Color::TrueColor {
                r: 128,
                g: 128,
                b: 128,
            }),
            error_prefix_color: input.error_prefix_color.unwrap_or(Color::Red),
            highlight_color: input.highlight_color.unwrap_or(Color::BrightBlue),
        }
    }
}
