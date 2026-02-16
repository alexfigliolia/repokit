use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ignore::WalkBuilder;

use crate::{
    executables::{
        intenal_executable::InternalExecutable, internal_executable_definition::RepoKitScope,
    },
    file_walker::walker::TSFileVisitorBuilder,
    internal_commands::{
        internal_registry::InternalRegistry, typescript_command::TypescriptCommand,
    },
    logger::logger::Logger,
    repokit::{interfaces::RepoKitCommand, repokit::RepoKit},
};

pub struct CommandValidations {
    scope: RepoKitScope,
}

impl CommandValidations {
    pub fn new(scope: &RepoKitScope) -> CommandValidations {
        CommandValidations {
            scope: scope.clone(),
        }
    }

    pub fn from(kit: &RepoKit) -> CommandValidations {
        CommandValidations {
            scope: kit.scope.clone(),
        }
    }

    pub fn collect_and_validate_internals(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals = InternalRegistry::new(&self.scope).get_all();
        self.detect_collisions_between_internals_and_root_commands(&internals);
        internals
    }

    pub fn collect_and_validate_externals(&self) -> HashMap<String, RepoKitCommand> {
        let paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let mut visitor = TSFileVisitorBuilder::new(&self.scope.root, &paths);
        WalkBuilder::new(&self.scope.root)
            .build_parallel()
            .visit(&mut visitor);
        let result = paths.lock().unwrap();
        let externals = TypescriptCommand::new(&self.scope.root).parse_commands(&result);
        let all = [&externals[..], &self.scope.configuration.thirdParty[..]].concat();
        self.detect_collisions_between_root_commands_and_externals(&all)
    }

    pub fn detect_collisions_between_internals_and_externals(
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, RepoKitCommand>,
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
            if self.scope.configuration.commands.contains_key(name) {
                Logger::info(
                    format!(
                        "I encountered a command named {} in your {} file that conflicts with one of my internals",
                        Logger::blue_bright(name),
                        Logger::blue_bright("repokit.ts"),
                    )
                    .as_str(),
                );
                Logger::exit_with_info("Please rename it");
            }
        }
    }

    fn detect_collisions_between_root_commands_and_externals(
        &self,
        externals: &Vec<RepoKitCommand>,
    ) -> HashMap<String, RepoKitCommand> {
        let mut map: HashMap<String, RepoKitCommand> = HashMap::new();
        for command in externals {
            if map.contains_key(&command.name) {
                let original = map.get(&command.name).expect("Unknown command");
                self.on_external_duplicate_collision(command, &original.location);
            }
            map.insert(command.name.clone(), command.clone());
            if self
                .scope
                .configuration
                .commands
                .contains_key(&command.name)
            {
                self.on_external_root_collision(command);
            }
        }
        map
    }

    fn on_external_root_collision(&self, command: &RepoKitCommand) {
        Logger::info(format!(
                "I encountered a package command named {} that conflicts with a command in your {} file",
                Logger::blue_bright(&command.name),
                Logger::blue_bright("repokit.ts")
            )
            .as_str(),
        );
        Logger::info("Here's where it's located:");
        Logger::log_file_path(&command.location);
        Logger::exit_with_info("Please rename one of these");
    }

    fn on_external_duplicate_collision(&self, command: &RepoKitCommand, collision_path: &str) {
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
