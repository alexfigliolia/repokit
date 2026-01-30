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
    pub fn new(root: String, configuration: DevKitConfig) -> CommandValidations {
        CommandValidations {
            root,
            configuration,
        }
    }

    pub fn from(kit: &DevKit) -> CommandValidations {
        CommandValidations {
            root: kit.root.clone(),
            configuration: kit.configuration.clone(),
        }
    }

    pub fn collect_and_validate_internals(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals =
            InternalRegistry::new(self.root.clone(), self.configuration.clone()).get_all();
        self.detect_collisions_between_internals_and_root_commands(&internals);
        internals
    }

    pub fn collect_and_validate_externals(&self) -> HashMap<String, DevKitCommand> {
        let finder = ExternalCommands::new(self.root.clone());
        let externals = executor::block_on(finder.find_all());
        self.detect_collisions_between_root_commands_and_externals(&externals)
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
        externals: &Vec<DevKitCommand>,
    ) -> HashMap<String, DevKitCommand> {
        let mut map: HashMap<String, DevKitCommand> = HashMap::new();
        for command in externals {
            if map.contains_key(&command.name) {
                let original = map.get(&command.name).expect("existent key");
                self.on_external_duplicate_collision(command, &original.location);
            }
            map.insert(command.name.clone(), command.clone());
            if self.configuration.commands.contains_key(&command.name) {
                self.on_external_root_collision(command);
            }
        }
        map
    }

    fn on_external_root_collision(&self, command: &DevKitCommand) {
        Logger::info(format!(
                "I encountered a package command named {} that conflicts with a command in your {} file",
                Logger::blue_bright(&command.name),
                Logger::blue_bright("devkit.ts")
            )
            .as_str(),
        );
        Logger::info("Here's where it's located:");
        Logger::log_file_path(&command.location);
        Logger::exit_with_info("Please rename one of these");
    }

    fn on_external_duplicate_collision(&self, command: &DevKitCommand, collision_path: &str) {
        Logger::info(
            format!(
                "I encountered two packages with the name {}",
                Logger::blue_bright(&command.name),
            )
            .as_str(),
        );
        Logger::info("Here's where they're located:\n");
        println!(
            "{}1. {}",
            Logger::indent(None),
            Logger::blue_bright(collision_path)
        );
        println!(
            "{}2. {}\n",
            Logger::indent(None),
            Logger::blue_bright(&command.location)
        );
        Logger::exit_with_info("Please rename one of these");
    }
}
