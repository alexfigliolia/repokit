use std::collections::HashMap;

use crate::{
    context::typescript_library_installation::CONFIG_FILE_NAME,
    executables::internal_executable::InternalExecutable,
    file_walker::file_walker::FileWalker,
    internal_commands::internal_registry::InternalCommandRegistry,
    logger::logger::Logger,
    repokit::{repokit_command::RepoKitCommand, repokit_runtime::RepoKitRuntime},
    typescript_library::typescript_bridge::TypeScriptBridge,
};

pub struct CommandValidations;

impl CommandValidations {
    pub fn collect_and_validate_internals() -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals = InternalCommandRegistry::new().get_all();
        CommandValidations::detect_collisions_between_internals_and_root_commands(&internals);
        internals
    }

    pub fn collect_and_validate_externals() -> HashMap<String, RepoKitCommand> {
        let walker = FileWalker::new();
        let result = walker.get();
        let externals = TypeScriptBridge::parse_commands(&result);
        let all = RepoKitRuntime::with_runtime(|runtime| {
            [&externals[..], &runtime.configuration.third_party[..]].concat()
        });
        CommandValidations::detect_collisions_between_root_commands_and_externals(&all)
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
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
    ) {
        for name in internals.keys() {
            if RepoKitRuntime::with_runtime(|runtime| {
                runtime.configuration.commands.contains_key(name)
            }) {
                Logger::info(
                    format!(
                        "I encountered a command named {} in your {} file that conflicts with one of my internals",
                        Logger::with_theme(|theme|theme.highlight(name)),
                        Logger::with_theme(|theme|theme.highlight(CONFIG_FILE_NAME)),
                    )
                    .as_str(),
                );
                Logger::exit_with_info("Please rename it");
            }
        }
    }

    fn detect_collisions_between_root_commands_and_externals(
        externals: &Vec<RepoKitCommand>,
    ) -> HashMap<String, RepoKitCommand> {
        RepoKitRuntime::with_runtime(|runtime| {
            let mut map: HashMap<String, RepoKitCommand> = HashMap::new();
            for command in externals {
                if map.contains_key(&command.name) {
                    let original = map.get(&command.name).expect("Unknown command");
                    CommandValidations::on_external_duplicate_collision(
                        command,
                        &original.location,
                    );
                }
                map.insert(command.name.clone(), command.clone());
                if runtime.configuration.commands.contains_key(&command.name) {
                    CommandValidations::on_external_root_collision(command);
                }
            }
            map
        })
    }

    fn on_external_root_collision(command: &RepoKitCommand) {
        Logger::info(format!(
                "I encountered a package command named {} that conflicts with a command in your {} file",
                Logger::with_theme(|theme|theme.highlight(&command.name)),
                Logger::with_theme(|theme|theme.highlight(CONFIG_FILE_NAME))
            )
            .as_str(),
        );
        Logger::info("Here's where it's located:");
        Logger::log_file_path(&command.location);
        Logger::exit_with_info("Please rename one of these");
    }

    fn on_external_duplicate_collision(command: &RepoKitCommand, collision_path: &str) {
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
