use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use jsonschema::Validator;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, from_value, to_value};

use crate::{
    configuration::recovery::Recovery,
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
    repokit::{
        command_definition::CommandDefinition,
        repokit_construct_validator::RepoKitConstructValidator,
    },
};

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct RepoKitCommand {
    pub name: String,
    pub owner: String,
    pub location: String,
    pub description: String,
    pub commands: HashMap<String, CommandDefinition>,
}

static REPOKIT_COMMAND_VALIDATOR: LazyLock<Mutex<Validator>> = LazyLock::new(|| {
    Mutex::new(Validator::new(&to_value(schemars::schema_for!(RepoKitCommand)).unwrap()).unwrap())
});

impl RepoKitCommand {
    fn register_encountered_errors(root: &str, failed_paths: Vec<String>) {
        let root_clone = root.to_string();
        PostProcessor::get().register_task(move || {
            println!();
            if !failed_paths.is_empty() {
                let appendage = if failed_paths.len() != 1 { "s" } else { "" };
                Logger::error(
                    format!(
                        "I encountered an error in the following command{}",
                        appendage
                    )
                    .as_str(),
                );
                Logger::list_file_paths(&failed_paths);
            } else {
                Logger::info("There was an error parsing one or more of your commands");
            }
            Logger::info("You can validate a command file's syntactical correctness by running");
            Logger::log_file_path(
                &Recovery::new(&root_clone).get_typecheck_command("<optional-path-to-file>"),
            );
        });
    }
}

impl RepoKitConstructValidator<Vec<Value>, Vec<RepoKitCommand>> for RepoKitCommand {
    fn from_input(root: &str, input: Vec<Value>) -> Vec<RepoKitCommand> {
        let mut result: Vec<RepoKitCommand> = Vec::new();
        let mut failures = 0;
        let mut failed_paths: Vec<String> = Vec::new();
        let validator = REPOKIT_COMMAND_VALIDATOR.lock().unwrap();
        for command in input {
            let repokit_command: Result<RepoKitCommand, serde_json::Error> =
                from_value(command.clone());
            if !RepoKitCommand::is_valid(&validator, &command) || repokit_command.is_err() {
                failures += 1;
                if let Some(path) = RepoKitCommand::on_parsing_error(root, command) {
                    failed_paths.push(path);
                }
            } else {
                let mut valid_command = repokit_command.expect("assertion success");
                valid_command.location = format!("{}/{}", root, valid_command.location);
                result.push(valid_command);
            }
        }
        if failures != 0 {
            RepoKitCommand::register_encountered_errors(root, failed_paths);
        }
        result
    }

    fn on_parsing_error(root: &str, command: Value) -> Option<String> {
        let location = command.get("location");
        println!();
        if location.is_some_and(|v| v.is_string()) {
            let path = format!("{}/{}", &root, location.unwrap().as_str().unwrap());
            return Some(path);
        }
        None
    }
}
