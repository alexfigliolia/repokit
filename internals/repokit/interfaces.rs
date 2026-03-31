use std::collections::HashMap;

use serde::Deserialize;

use crate::themes::theme_inputs::RepoKitTheme;

#[derive(Debug, Deserialize, Clone)]
pub struct CommandDefinition {
    pub command: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RootCommand {
    pub name: String,
    pub command: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}

impl RootCommand {
    pub fn from(name: &str, command: &CommandDefinition) -> RootCommand {
        RootCommand {
            name: name.to_string(),
            args: command.args.clone(),
            command: command.command.to_string(),
            description: command.description.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RepoKitConfig {
    pub project: String,
    pub thirdParty: Vec<RepoKitCommand>,
    pub commands: HashMap<String, CommandDefinition>,
    pub themes: Vec<RepoKitTheme>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RepoKitCommand {
    pub name: String,
    pub owner: String,
    pub location: String,
    pub description: String,
    pub commands: HashMap<String, CommandDefinition>,
}
