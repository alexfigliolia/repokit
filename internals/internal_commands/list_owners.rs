use std::collections::{HashMap, HashSet};

use alphanumeric_sort::sort_str_slice;

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::repokit_command::RepoKitCommand,
    validations::command_validations::CommandValidations,
};

pub struct ListOwners {
    pub definition: InternalExecutableDefinition,
}

impl ListOwners {
    pub fn new() -> ListOwners {
        ListOwners {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "owners",
                description: "Lists all registered command owners",
                args: [],
            }),
        }
    }

    fn collect_registered_commands(&self) -> HashMap<String, RepoKitCommand> {
        let mut validators = CommandValidations::new();
        validators.collect_and_validate_externals()
    }
}

impl InternalExecutable for ListOwners {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        let registered_commands = self.collect_registered_commands();
        Logger::info("Listing all command owners");
        let mut owners: HashSet<String> = HashSet::new();
        for (_, command) in registered_commands {
            if !command.owner.is_empty() {
                owners.insert(command.owner);
            }
        }
        if owners.is_empty() {
            return Logger::exit_with_info("No owners found");
        }
        let mut list: Vec<String> = owners.into_iter().collect();
        Logger::with_surrounding_space(|| {
            sort_str_slice(&mut list);
            for (index, owner) in list.iter().enumerate() {
                println!(
                    "{}{}",
                    Logger::indent(None),
                    Logger::cyan(format!("{}. {}", index + 1, &owner).as_str()),
                );
            }
        });
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
