use crate::themes::theme::{RepoKitTheme, Theme, ThemeInputColors};

pub struct ThemeRegistry {
    pub theme: Theme,
}

impl ThemeRegistry {
    pub fn new() -> ThemeRegistry {
        ThemeRegistry {
            theme: ThemeRegistry::default_theme(),
        }
    }

    pub fn register(&mut self, theme: &RepoKitTheme) {
        self.theme = Theme::from_configuration(theme);
    }

    fn default_theme() -> Theme {
        Theme::new(ThemeInputColors {
            prefixColor: None,
            commandColor: None,
            subcommandColor: None,
            argColor: None,
            descriptionColor: None,
            errorPrefixColor: None,
            highlightColor: None,
        })
    }
}
