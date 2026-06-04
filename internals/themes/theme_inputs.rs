use colored::Color;
use schemars::JsonSchema;
use serde::Deserialize;

pub struct ThemeInputColors {
    pub prefix_color: Option<Color>,
    pub command_color: Option<Color>,
    pub subcommand_color: Option<Color>,
    pub arg_color: Option<Color>,
    pub description_color: Option<Color>,
    pub error_prefix_color: Option<Color>,
    pub highlight_color: Option<Color>,
}

pub struct ThemeInput {
    pub name: String,
    pub colors: ThemeInputColors,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct RepoKitThemeColors {
    #[serde(rename = "prefixColor")]
    pub prefix_color: Option<String>,
    #[serde(rename = "commandColor")]
    pub command_color: Option<String>,
    #[serde(rename = "subcommandColor")]
    pub subcommand_color: Option<String>,
    #[serde(rename = "argColor")]
    pub arg_color: Option<String>,
    #[serde(rename = "descriptionColor")]
    pub description_color: Option<String>,
    #[serde(rename = "errorPrefixColor")]
    pub error_prefix_color: Option<String>,
    #[serde(rename = "highlightColor")]
    pub highlight_color: Option<String>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct RepoKitTheme {
    pub name: String,
    pub colors: RepoKitThemeColors,
}
