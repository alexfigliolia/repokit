use std::collections::HashMap;

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct CommandDefinition {
    pub command: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}
