use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ParsedCommand {
    pub name: String,
    pub command: String,
    pub description: String,
}

impl ParsedCommand {
    pub fn from(name: &str, command: &Command) -> ParsedCommand {
        ParsedCommand {
            name: name.to_string(),
            command: command.command.to_string(),
            description: command.description.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DevKitConfig {
    pub project: String,
    pub workspaces: Vec<String>,
    pub commands: HashMap<String, Command>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DevKitCommand {
    pub name: String,
    pub location: String,
    pub description: String,
    pub commands: HashMap<String, Command>,
}
