use colored::{Color, ColoredString, Colorize};

use crate::themes::{
    theme_colors::ThemeColors,
    theme_inputs::{RepoKitTheme, ThemeInput, ThemeInputColors},
};

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ThemeColors,
}

impl Theme {
    pub fn new(input: ThemeInput) -> Theme {
        Theme {
            name: input.name,
            colors: ThemeColors {
                prefix_color: input.colors.prefix_color.unwrap_or(Color::BrightMagenta),
                command_color: input.colors.command_color.unwrap_or(Color::BrightBlue),
                subcommand_color: input.colors.subcommand_color.unwrap_or(Color::TrueColor {
                    r: 175,
                    g: 247,
                    b: 7,
                }),
                arg_color: input.colors.arg_color.unwrap_or(Color::Green),
                description_color: input.colors.description_color.unwrap_or(Color::TrueColor {
                    r: 128,
                    g: 128,
                    b: 128,
                }),
                error_prefix_color: input.colors.error_prefix_color.unwrap_or(Color::Red),
                highlight_color: input.colors.highlight_color.unwrap_or(Color::BrightBlue),
            },
        }
    }

    pub fn prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.prefix_color).bold()
    }

    pub fn command(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.command_color)
    }

    pub fn sub_command(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.subcommand_color)
    }

    pub fn arg(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.arg_color)
    }

    pub fn description(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.description_color)
    }

    pub fn error_prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.error_prefix_color).bold()
    }

    pub fn highlight(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.highlight_color)
    }

    pub fn from_configuration(theme: &RepoKitTheme) -> Theme {
        Theme::new(ThemeInput {
            name: theme.name.clone(),
            colors: ThemeInputColors {
                prefix_color: Theme::parse_rgb(&theme.colors.prefix_color),
                command_color: Theme::parse_rgb(&theme.colors.command_color),
                subcommand_color: Theme::parse_rgb(&theme.colors.subcommand_color),
                arg_color: Theme::parse_rgb(&theme.colors.arg_color),
                description_color: Theme::parse_rgb(&theme.colors.description_color),
                error_prefix_color: Theme::parse_rgb(&theme.colors.error_prefix_color),
                highlight_color: Theme::parse_rgb(&theme.colors.highlight_color),
            },
        })
    }

    fn parse_rgb(rgb_str: &Option<String>) -> Option<Color> {
        match rgb_str {
            Some(rgb) => {
                let trimmed = rgb
                    .strip_prefix("rgb(")
                    .and_then(|s| s.strip_suffix(')'))
                    .map(|s| s.trim())?;

                // 2. Split the remaining string by commas.
                let parts: Vec<&str> = trimmed.split(',').collect();

                if parts.len() == 3 {
                    // 3. Trim whitespace from each part and parse to a u8.
                    //    `.parse()` returns a `Result`, so we use `.ok()` to convert to an `Option`,
                    //    and the `?` operator to return early if any parse fails.
                    let r = parts[0].trim().parse::<u8>().ok()?;
                    let g = parts[1].trim().parse::<u8>().ok()?;
                    let b = parts[2].trim().parse::<u8>().ok()?;

                    Some(Color::TrueColor { r, g, b })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
