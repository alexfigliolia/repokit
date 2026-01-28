use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub command: String,
    pub description: String,
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
