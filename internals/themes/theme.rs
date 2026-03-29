use colored::{Color, ColoredString, Colorize};
use serde::Deserialize;

pub struct Theme {
    prefixColor: Color,
    commandColor: Color,
    subcommandColor: Color,
    argColor: Color,
    descriptionColor: Color,
    errorPrefixColor: Color,
    highlightColor: Color,
}

impl Theme {
    pub fn new(colors: ThemeInputColors) -> Theme {
        Theme::from(colors)
    }

    pub fn prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.prefixColor)
    }

    pub fn command(&self, msg: &str) -> ColoredString {
        msg.color(self.commandColor)
    }

    pub fn sub_command(&self, msg: &str) -> ColoredString {
        msg.color(self.subcommandColor)
    }

    pub fn arg(&self, msg: &str) -> ColoredString {
        msg.color(self.argColor)
    }

    pub fn description(&self, msg: &str) -> ColoredString {
        msg.color(self.descriptionColor)
    }

    pub fn error_prefix(&self, msg: &str) -> ColoredString {
        msg.color(self.errorPrefixColor)
    }

    pub fn highlight(&self, msg: &str) -> ColoredString {
        msg.color(self.highlightColor)
    }

    pub fn from_configuration(theme: &RepoKitTheme) -> Theme {
        let copy = theme.clone();
        Theme::new(ThemeInputColors {
            prefixColor: Theme::parse_rgb(copy.prefixColor),
            commandColor: Theme::parse_rgb(copy.commandColor),
            subcommandColor: Theme::parse_rgb(copy.subcommandColor),
            argColor: Theme::parse_rgb(copy.argColor),
            descriptionColor: Theme::parse_rgb(copy.descriptionColor),
            errorPrefixColor: Theme::parse_rgb(copy.errorPrefixColor),
            highlightColor: Theme::parse_rgb(copy.highlightColor),
        })
    }

    fn parse_rgb(rgb_str: Option<String>) -> Option<Color> {
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

pub struct ThemeInputColors {
    pub prefixColor: Option<Color>,
    pub commandColor: Option<Color>,
    pub subcommandColor: Option<Color>,
    pub argColor: Option<Color>,
    pub descriptionColor: Option<Color>,
    pub errorPrefixColor: Option<Color>,
    pub highlightColor: Option<Color>,
}

impl From<ThemeInputColors> for Theme {
    fn from(input: ThemeInputColors) -> Theme {
        Theme {
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

#[derive(Debug, Deserialize, Clone)]
pub struct RepoKitTheme {
    pub prefixColor: Option<String>,
    pub commandColor: Option<String>,
    pub subcommandColor: Option<String>,
    pub argColor: Option<String>,
    pub descriptionColor: Option<String>,
    pub errorPrefixColor: Option<String>,
    pub highlightColor: Option<String>,
}
