use std::collections::HashMap;

use futures::executor;

use crate::{
    devkit::{
        devkit::DevKit,
        interfaces::{DevKitCommand, DevKitConfig},
    },
    executables::intenal_executable::InternalExecutable,
    external_commands::external_commands::ExternalCommands,
    internal_commands::internal_registry::InternalRegistry,
    logger::logger::Logger,
};

pub struct CommandValidations {
    root: String,
    configuration: DevKitConfig,
}

impl CommandValidations {
    pub fn from(kit: &DevKit) -> CommandValidations {
        CommandValidations {
            root: kit.root.clone(),
            configuration: kit.configuration.clone(),
        }
    }

    pub fn collect_and_validate_internals(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals = InternalRegistry::new(self.root.clone()).get_all();
        self.detect_collisions_between_internals_and_root_commands(&internals);
        internals
    }

    pub fn collect_and_validate_externals(&self) -> HashMap<String, DevKitCommand> {
        let finder = ExternalCommands::new(self.root.clone());
        let externals = executor::block_on(finder.find_all());
        self.detect_collisions_between_root_commands_and_externals(&externals);
        externals
    }

    pub fn detect_collisions_between_internals_and_externals(
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        for (name, command) in externals {
            if internals.contains_key(name) {
                Logger::info(
                    format!(
                        "I encountered a command named {} that conflicts with one of my internals",
                        Logger::blue_bright(name),
                    )
                    .as_str(),
                );
                Logger::info("Here's where it's located:");
                Logger::log_file_path(&command.location);
                Logger::exit_with_info("Please rename it");
            }
        }
    }

    fn detect_collisions_between_internals_and_root_commands(
        &self,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
    ) {
        for name in internals.keys() {
            if self.configuration.commands.contains_key(name) {
                Logger::info(
                    format!(
                        "I encountered a command named {} in your {} file that conflicts with one of my internals",
                        Logger::blue_bright(name),
                        Logger::blue_bright("devkit.ts"),
                    )
                    .as_str(),
                );
                Logger::exit_with_info("Please rename it");
            }
        }
    }

    fn detect_collisions_between_root_commands_and_externals(
        &self,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        for (name, command) in externals {
            if self.configuration.commands.contains_key(name) {
                Logger::info(
                    format!(
                        "I encountered a package command named {} that conflicts with a command in your {} file",
                        Logger::blue_bright(name),
                        Logger::blue_bright("devkit.ts")
                    )
                    .as_str(),
                );
                Logger::info("Here's where it's located:");
                Logger::log_file_path(&command.location);
                Logger::exit_with_info("Please rename one of these");
            }
        }
    }
}
