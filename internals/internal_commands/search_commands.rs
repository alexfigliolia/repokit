use alphanumeric_sort::sort_slice_by_str_key;
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
    repokit::{
        command_definition::CommandDefinition, repokit_command::RepoKitCommand,
        repokit_runtime::RepoKitRuntime,
    },
    validations::command_validations::CommandValidations,
};

pub struct SearchCommands {
    pub definition: InternalExecutableDefinition,
}

impl SearchCommands {
    pub fn new() -> SearchCommands {
        SearchCommands {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "search",
                description: "Retrieve commands that match any search query",
                args: [(
                    "<query>",
                    "A search string to match against command names, descriptions, arguments, or owner",
                )],
            }),
        }
    }

    fn search_internal(&self, query: &str, command: &Box<dyn InternalExecutable>) -> bool {
        let config = command.get_definition();
        if config.name.to_lowercase().contains(query) {
            return true;
        }
        if config.description.to_lowercase().contains(query) {
            return true;
        }
        if let Some(args) = &config.args {
            for (arg, description) in args {
                if arg.to_lowercase().contains(query) || description.to_lowercase().contains(query)
                {
                    return true;
                }
            }
        }

        false
    }

    fn search_external(&self, query: &str, command: &RepoKitCommand) -> bool {
        if command.name.to_lowercase().contains(query) {
            return true;
        }
        if command.owner.to_lowercase().contains(query)
            || command.location.to_lowercase().contains(query)
            || command.description.to_lowercase().contains(query)
        {
            return true;
        }
        for (command_name, definition) in &command.commands {
            if command_name.to_lowercase().contains(query) || self.search_command(query, definition)
            {
                return true;
            }
        }
        false
    }

    fn search_command(&self, query: &str, definition: &CommandDefinition) -> bool {
        if definition.command.to_lowercase().contains(query)
            || definition.description.to_lowercase().contains(query)
        {
            return true;
        }
        if let Some(args) = &definition.args {
            for (flag, description) in args {
                if flag.to_lowercase().contains(query) || description.to_lowercase().contains(query)
                {
                    return true;
                }
            }
        }
        false
    }

    fn log_root_results(&self, root_results: &HashMap<String, CommandDefinition>) {
        let total = root_results.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !root_results.is_empty() {
            Help::log_root_commands(root_results);
        }
        Logger::info(
            format!(
                "Matched {} command{} in your repokit config",
                Logger::with_theme(|theme| theme.highlight(total.to_string().as_str())),
                plural_appendage,
            )
            .as_str(),
        );
    }

    fn log_internal_results(
        &self,
        internal_results: &HashMap<String, &Box<dyn InternalExecutable>>,
    ) {
        let total = internal_results.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !internal_results.is_empty() {
            let mut sorted_internals: Vec<&&Box<dyn InternalExecutable>> =
                internal_results.values().collect();
            sort_slice_by_str_key(&mut sorted_internals, |x| &x.get_definition().name);
            Logger::space_around("Internal Commands:");
            for internal in sorted_internals {
                internal.help();
                println!();
            }
        }
        Logger::info(
            format!(
                "Matched {} internal command{}",
                Logger::with_theme(|theme| theme.highlight(total.to_string().as_str())),
                plural_appendage,
            )
            .as_str(),
        );
    }

    fn log_external_results(&self, external_commands: &HashMap<String, RepoKitCommand>) {
        let total = external_commands.len();
        let plural_appendage = if total == 1 { "" } else { "s" };
        if !external_commands.is_empty() {
            Help::log_external_commands(external_commands);
        }
        Logger::info(
            format!(
                "Matched {} registered command{}",
                Logger::with_theme(|theme| theme.highlight(total.to_string().as_str())),
                plural_appendage,
            )
            .as_str(),
        );
    }
}

impl InternalExecutable for SearchCommands {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Searching commands");
        if args.is_empty() {
            Logger::exit_with_error("Please specify a search string to query with");
        }
        let query = args.join(" ").to_lowercase();
        let externals = CommandValidations::collect_and_validate_externals();
        let mut root_results: HashMap<String, CommandDefinition> = HashMap::new();
        let mut internal_results: HashMap<String, &Box<dyn InternalExecutable>> = HashMap::new();
        let mut external_results: HashMap<String, RepoKitCommand> = HashMap::new();
        RepoKitRuntime::with_runtime(|runtime| {
            for (command, script) in &runtime.configuration.commands {
                if self.search_command(&query, script) {
                    root_results.insert(command.clone(), script.clone());
                }
            }
        });
        for (name, command) in internals {
            if self.search_internal(&query, command) {
                internal_results.insert(name.clone(), command);
            }
        }
        for (name, command) in externals {
            if self.search_external(&query, &command) {
                external_results.insert(name, command);
            }
        }
        if root_results.is_empty() && internal_results.is_empty() && external_results.is_empty() {
            Logger::exit_with_info("No matched commands");
        }
        self.log_root_results(&root_results);
        self.log_internal_results(&internal_results);
        self.log_external_results(&external_results);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
