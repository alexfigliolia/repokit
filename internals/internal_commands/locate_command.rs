use std::collections::HashMap;

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
    validations::command_validations::CommandValidations,
};

pub struct LocateCommand {
    pub definition: InternalExecutableDefinition,
}

impl LocateCommand {
    pub fn new() -> LocateCommand {
        LocateCommand {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "locate",
                description: "Locates command definitions",
                args: [("<name>", "The name of a registered command")],
            }),
        }
    }

    fn search_externals(&self, query: &str) {
        let all = CommandValidations::collect_and_validate_externals();
        for (_, command) in all {
            if command.name == query {
                Logger::log_file_path(&command.location);
                panic!();
            }
        }
    }

    fn search_root(&self, command: &str) {
        RepoKitRuntime::with_runtime(|runtime| {
            if runtime.configuration.commands.contains_key(command) {
                Logger::log_file_path(format!("{}/repokit.ts", &runtime.git.root).as_str());
                panic!();
            }
        });
    }
}

impl InternalExecutable for LocateCommand {
    fn run(&self, args: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        if args.is_empty() {
            Logger::exit_with_info("Please specify a command to locate");
        }
        let command = &args[0];
        Logger::info(
            format!(
                "Locating a command named {}",
                Logger::with_theme(|theme| theme.highlight(command))
            )
            .as_str(),
        );
        self.search_externals(command);
        self.search_root(command);
        Logger::exit_with_error(
            format!(
                "I could not find a command named {}",
                Logger::with_theme(|theme| theme.highlight(command))
            )
            .as_str(),
        );
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
