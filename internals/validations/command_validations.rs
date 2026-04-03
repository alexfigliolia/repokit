use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ignore::WalkBuilder;

use crate::{
    executables::{
        internal_executable::InternalExecutable, internal_executable_definition::RepoKitScope,
    },
    file_walker::walker::TSFileVisitorBuilder,
    internal_commands::internal_registry::InternalRegistry,
    logger::logger::Logger,
    repokit::{repokit::RepoKit, repokit_command::RepoKitCommand},
};

pub struct CommandValidations {
    pub scope: RepoKitScope,
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
        let root = &self.scope.git.root;
        let paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        if let Some(files) = &self.scope.cache.files_to_crawl {
            let mut paths_to_search = paths.lock().unwrap();
            for file in files {
                paths_to_search.push(file.to_owned())
            }
        } else {
            let mut visitor = TSFileVisitorBuilder::new(root, &paths);
            WalkBuilder::new(root).build_parallel().visit(&mut visitor);
            self.scope.cache.store_crawl_cache(
                &self.scope.git.commit_hash,
                paths.lock().unwrap().join("\n"),
            );
        }
        let result = paths.lock().unwrap();
        let externals = self.scope.bridge.parse_commands(&result);
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
                        Logger::with_theme(|theme| theme.highlight(name)),
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
                        Logger::with_theme(|theme|theme.highlight(name)),
                        Logger::with_theme(|theme|theme.highlight("repokit.ts")),
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
                Logger::with_theme(|theme|theme.highlight(&command.name)),
                Logger::with_theme(|theme|theme.highlight("repokit.ts"))
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
                Logger::with_theme(|theme| theme.highlight(&command.name)),
            )
            .as_str(),
        );
        Logger::info("Here's where they're located:\n");
        println!(
            "{}1. {}",
            Logger::indent(None),
            Logger::with_theme(|theme| theme.highlight(collision_path))
        );
        println!(
            "{}2. {}\n",
            Logger::indent(None),
            Logger::with_theme(|theme| theme.highlight(&command.location))
        );
        Logger::exit_with_info("Please rename one of these");
    }
}
