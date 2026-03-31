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
                prefixColor: input.colors.prefixColor.unwrap_or(Color::BrightMagenta),
                commandColor: input.colors.commandColor.unwrap_or(Color::BrightBlue),
                subcommandColor: input.colors.subcommandColor.unwrap_or(Color::TrueColor {
                    r: 175,
                    g: 247,
                    b: 7,
                }),
                argColor: input.colors.argColor.unwrap_or(Color::Green),
                descriptionColor: input.colors.descriptionColor.unwrap_or(Color::TrueColor {
                    r: 128,
                    g: 128,
                    b: 128,
                }),
                errorPrefixColor: input.colors.errorPrefixColor.unwrap_or(Color::Red),
                highlightColor: input.colors.highlightColor.unwrap_or(Color::BrightBlue),
            },
        }
    }

    pub fn prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.prefixColor).bold()
    }

    pub fn command(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.commandColor)
    }

    pub fn sub_command(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.subcommandColor)
    }

    pub fn arg(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.argColor)
    }

    pub fn description(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.descriptionColor)
    }

    pub fn error_prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.errorPrefixColor).bold()
    }

    pub fn highlight(&self, msg: &str) -> ColoredString {
        msg.color(self.colors.highlightColor)
    }

    pub fn from_configuration(theme: &RepoKitTheme) -> Theme {
        Theme::new(ThemeInput {
            name: theme.name.clone(),
            colors: ThemeInputColors {
                prefixColor: Theme::parse_rgb(&theme.colors.prefixColor),
                commandColor: Theme::parse_rgb(&theme.colors.commandColor),
                subcommandColor: Theme::parse_rgb(&theme.colors.subcommandColor),
                argColor: Theme::parse_rgb(&theme.colors.argColor),
                descriptionColor: Theme::parse_rgb(&theme.colors.descriptionColor),
                errorPrefixColor: Theme::parse_rgb(&theme.colors.errorPrefixColor),
                highlightColor: Theme::parse_rgb(&theme.colors.highlightColor),
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
