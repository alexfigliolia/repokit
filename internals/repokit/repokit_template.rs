use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::repokit::command_definition::CommandDefinition;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RepoKitTemplate {
    pub name: String,
    pub owner: String,
    pub description: String,
    pub commands: HashMap<String, CommandDefinition>,
}
