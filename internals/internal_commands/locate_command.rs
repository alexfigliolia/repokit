use std::collections::HashMap;

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput, RepoKitScope,
        },
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
    validations::command_validations::CommandValidations,
};

pub struct LocateCommand {
    pub scope: RepoKitScope,
    pub definition: InternalExecutableDefinition,
}

impl LocateCommand {
    pub fn new(scope: &RepoKitScope) -> LocateCommand {
        LocateCommand {
            scope: scope.clone(),
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "locate",
                description: "Locates command definitions",
                args: [("<name>", "The name of a registered command")],
            }),
        }
    }

    fn search_externals(&self, query: &str) {
        let finder = CommandValidations::new(&self.scope);
        let all = finder.collect_and_validate_externals();
        for (_, command) in all {
            if command.name == query {
                Logger::log_file_path(&command.location);
                PostProcessor::get().flush();
            }
        }
    }

    fn search_root(&self, command: &str) {
        if self.scope.configuration.commands.contains_key(command) {
            Logger::log_file_path(format!("{}/repokit.ts", &self.scope.root).as_str());
            PostProcessor::get().flush();
        }
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

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
