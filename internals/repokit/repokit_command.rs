use std::{collections::HashMap, sync::LazyLock};

use jsonschema::Validator;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, from_value, to_value};

use crate::{
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
    repokit::{
        command_definition::CommandDefinition,
        repokit_construct_validator::RepoKitConstructValidator, repokit_runtime::RepoKitRuntime,
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

static REPOKIT_COMMAND_VALIDATOR: LazyLock<Validator> = LazyLock::new(|| {
    Validator::new(&to_value(schemars::schema_for!(RepoKitCommand)).unwrap()).unwrap()
});

impl RepoKitConstructValidator for RepoKitCommand {}

impl RepoKitCommand {
    pub fn from_input(input: Vec<Value>) -> Vec<RepoKitCommand> {
        let mut result: Vec<RepoKitCommand> = Vec::new();
        let mut failures = 0;
        let mut failed_paths: Vec<String> = Vec::new();
        let install_path = RepoKitRuntime::with_runtime(|runtime| {
            runtime
                .typescript_library
                .install_path
                .to_string_lossy()
                .to_string()
        });
        for command in input {
            let repokit_command: Result<RepoKitCommand, serde_json::Error> =
                from_value(command.clone());
            if !RepoKitCommand::is_valid(&REPOKIT_COMMAND_VALIDATOR, &command)
                || repokit_command.is_err()
            {
                failures += 1;
                if let Some(path) = RepoKitCommand::on_parsing_error(command) {
                    failed_paths.push(path);
                }
            } else {
                let mut valid_command = repokit_command.unwrap();
                valid_command.location = format!("{}/{}", install_path, valid_command.location);
                result.push(valid_command);
            }
        }
        if failures != 0 {
            RepoKitCommand::register_encountered_errors(failed_paths);
        }
        result
    }

    pub fn on_parsing_error(command: Value) -> Option<String> {
        let location = command.get("location");
        println!();
        if location.is_some_and(|v| v.is_string()) {
            return Some(RepoKitRuntime::with_runtime(|runtime| {
                runtime
                    .typescript_library
                    .install_path
                    .join(location.unwrap().as_str().unwrap())
                    .to_string_lossy()
                    .to_string()
            }));
        }
        None
    }

    fn register_encountered_errors(failed_paths: Vec<String>) {
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
            let type_check_command = RepoKitRuntime::with_runtime(|mut runtime| {
                runtime
                    .node
                    .get_typecheck_command("<optional-path-to-file>")
            });
            Logger::log_file_path(&type_check_command);
        });
    }
}
