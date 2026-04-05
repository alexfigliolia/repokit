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
    repokit::{repokit_command::RepoKitCommand, repokit_runtime::RepoKitRuntime},
    validations::command_validations::CommandValidations,
};

pub struct ListCommands {
    pub definition: InternalExecutableDefinition,
}

static SCOPES: [&str; 4] = ["internal", "registered", "root", "<owner>"];

impl ListCommands {
    pub fn new() -> ListCommands {
        ListCommands {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "list",
                description: "List commands based on their scope of definition",
                args: [(
                    "<scope>",
                    format!(
                        "The scope of the commands you wish to list. Specify one of {}",
                        Logger::with_theme(|theme| theme.highlight(SCOPES.join(" | ").as_str()))
                    )
                    .as_str(),
                )],
            }),
        }
    }

    fn exit_on_invalid_scope(&self) {
        Logger::exit_with_info(
            format!(
                "Please specify a scope to list the commands of. Select one of {}",
                Logger::with_theme(|theme| theme.highlight(SCOPES.join(" | ").as_str()))
            )
            .as_str(),
        );
    }
}

impl InternalExecutable for ListCommands {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        if args.is_empty() {
            return self.exit_on_invalid_scope();
        }
        let query = args[0].as_str();
        let scope = &query.to_lowercase();
        if scope == SCOPES[0] {
            return Help::log_internal_commands(internals);
        }
        if scope == SCOPES[2] {
            return RepoKitRuntime::with_runtime(|runtime| {
                Help::log_root_commands(&runtime.configuration.commands)
            });
        }
        let registered_commands = CommandValidations::collect_and_validate_externals();
        if scope == SCOPES[1] {
            return Help::log_external_commands(&registered_commands);
        }
        let full_query = args.join(" ");
        let full_scope = &full_query.to_lowercase();
        Logger::info("Searching registered commands");
        let matches: HashMap<String, RepoKitCommand> = registered_commands
            .iter()
            .filter_map(|(name, x)| {
                if x.owner.to_lowercase().contains(full_scope) {
                    return Some((name.clone(), x.clone()));
                }
                None
            })
            .collect();
        if matches.is_empty() {
            Logger::exit_with_info(
                format!(
                    "I could not find any commands matching {}",
                    Logger::with_theme(|theme| theme.highlight(&full_query))
                )
                .as_str(),
            );
        }
        Help::log_external_commands(&matches);
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
