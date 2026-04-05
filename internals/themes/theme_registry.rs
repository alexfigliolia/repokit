use std::collections::HashMap;

use crate::themes::{
        built_in_themes::{money::MONEY, seeing_red::SEEING_RED, the_blues::THE_BLUES},
        theme::Theme,
        theme_colors::ThemeColors,
        theme_inputs::{RepoKitTheme, ThemeInputColors},
    };

pub struct ThemeRegistry {
    pub theme: String,
    pub default_theme: String,
    pub themes: HashMap<String, Theme>,
}

impl ThemeRegistry {
    pub fn new() -> ThemeRegistry {
        let (default_theme_name, built_in_themes) = ThemeRegistry::built_in_themes();
        ThemeRegistry {
            themes: HashMap::from(built_in_themes),
            theme: default_theme_name.to_string(),
            default_theme: default_theme_name.to_string(),
        }
    }

    pub fn set_theme(&mut self, theme: &str) -> bool {
        if self.themes.contains_key(theme) && self.theme != theme {
            self.theme = theme.to_string();
            return true;
        }
        false
    }

    pub fn current_theme(&self) -> &Theme {
        if self.themes.contains_key(&self.theme) {
            return self
                .themes
                .get(&self.theme)
                .expect("the current theme was not found");
        }
        self.themes
            .get(&self.default_theme)
            .expect("default theme should always exist")
    }

    pub fn register_user_theme(&mut self, theme: &RepoKitTheme) {
        if self.themes.contains_key(&theme.name) {
            self.on_naming_conflict(&theme.name);
        }
        self.themes
            .insert(theme.name.clone(), Theme::from_configuration(theme));
    }

    pub fn register_theme(&mut self, theme: Theme) {
        let name = theme.name.clone();
        self.themes.insert(name, theme);
    }

    pub fn has(&self, theme: &str) -> bool {
        self.themes.contains_key(theme)
    }

    fn built_in_themes() -> (String, [(String, Theme); 4]) {
        let (default_theme_name, built_in_color_schemes) = ThemeRegistry::built_in_color_schemes();
        (
            default_theme_name,
            built_in_color_schemes.map(|(name, colors)| {
                (
                    name.to_string(),
                    Theme {
                        colors,
                        name: name.to_string(),
                    },
                )
            }),
        )
    }

    fn built_in_color_schemes() -> (String, [(&'static str, ThemeColors); 4]) {
        let default_theme_name = "default";
        (
            default_theme_name.to_string(),
            [
                (default_theme_name, ThemeRegistry::create_default()),
                ("seeing-red", SEEING_RED),
                ("the-blues", THE_BLUES),
                ("money", MONEY),
            ],
        )
    }

    fn create_default() -> ThemeColors {
        ThemeColors::from_options(ThemeInputColors {
            prefixColor: None,
            commandColor: None,
            subcommandColor: None,
            argColor: None,
            descriptionColor: None,
            errorPrefixColor: None,
            highlightColor: None,
        })
    }

    fn on_naming_conflict(&self, name: &str) {
        if ThemeRegistry::built_in_color_schemes()
            .1
            .map(|t| t.0)
            .contains(&name)
        {
            println!(
                "{}: I've discovered a theme that conflicts with one of my iternals. Please rename your {} theme",
                self.current_theme().error_prefix("RepoKit"),
                self.current_theme().highlight(name)
            )
        } else {
            println!(
                "{}: I've discovered two themes with the name {}. Please rename one of them",
                self.current_theme().error_prefix("RepoKit"),
                self.current_theme().highlight(name)
            )
        }
    }
}
