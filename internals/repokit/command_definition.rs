use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CommandDefinition {
    pub command: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}
