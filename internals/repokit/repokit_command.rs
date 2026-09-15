use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    logger::logger::Logger,
    repokit::{command_definition::CommandDefinition, repokit_runtime::RepoKitRuntime},
};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct RepoKitCommand {
    pub name: String,
    pub owner: String,
    pub location: String,
    pub description: String,
    pub commands: HashMap<String, CommandDefinition>,
}

impl RepoKitCommand {
    pub fn resolve_paths(mut input: Vec<RepoKitCommand>) -> Vec<RepoKitCommand> {
        let install_path = RepoKitRuntime::with_runtime(|runtime| {
            runtime
                .typescript_library
                .install_path
                .to_string_lossy()
                .to_string()
        });
        for command in &mut input {
            command.location = format!("{}/{}", install_path, command.location);
        }
        input
    }

    pub fn on_parsing_error(failed_path: &str) {
        println!();
        Logger::error("I encountered an error when parsing the following command");
        Logger::info("This can occur if a command file throws an error upon evaluation");
        Logger::log_file_path(failed_path);
        panic!();
    }
}
