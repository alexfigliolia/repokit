use colored::Color;
use serde::Deserialize;

pub struct ThemeInputColors {
    pub prefixColor: Option<Color>,
    pub commandColor: Option<Color>,
    pub subcommandColor: Option<Color>,
    pub argColor: Option<Color>,
    pub descriptionColor: Option<Color>,
    pub errorPrefixColor: Option<Color>,
    pub highlightColor: Option<Color>,
}

pub struct ThemeInput {
    pub name: String,
    pub colors: ThemeInputColors,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RepoKitThemeColors {
    pub prefixColor: Option<String>,
    pub commandColor: Option<String>,
    pub subcommandColor: Option<String>,
    pub argColor: Option<String>,
    pub descriptionColor: Option<String>,
    pub errorPrefixColor: Option<String>,
    pub highlightColor: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RepoKitTheme {
    pub name: String,
    pub colors: RepoKitThemeColors,
}
