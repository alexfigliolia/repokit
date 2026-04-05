use std::collections::HashMap;

use alphanumeric_sort::{sort_slice_by_str_key, sort_str_slice};

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    logger::logger::Logger,
    repokit::{
        command_definition::CommandDefinition, repokit_command::RepoKitCommand,
        repokit_config::RootCommand, repokit_runtime::RepoKitRuntime,
    },
};

pub struct Help;

impl Help {
    pub fn list_all(
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, RepoKitCommand>,
    ) {
        Help::log_internal_commands(internals);
        RepoKitRuntime::with_runtime(|runtime| {
            Help::log_root_commands(&runtime.configuration.commands);
        });
        Help::log_external_commands(externals);
    }

    pub fn log_internal_command(command: &InternalExecutableDefinition) {
        Logger::with_theme(|theme| {
            println!(
                "{}{} {}",
                Logger::indent(Some(3)),
                theme.command(&command.name),
                theme.description(&command.description),
            );
        });
        Help::log_args(&command.args, None);
    }

    pub fn log_root_command(command: &RootCommand) {
        Logger::with_theme(|theme| {
            println!(
                "{}{} {}",
                Logger::indent(Some(3)),
                theme.command(&command.name),
                theme.description(&command.description),
            );
        });
        Help::log_args(&command.args, None)
    }

    pub fn log_external_command(command: &RepoKitCommand) {
        Logger::with_theme(|theme| {
            println!(
                "{}{} {}",
                Logger::indent(Some(3)),
                theme.command(&command.name),
                theme.description(&command.description),
            );
        });
        println!();
        Help::log_external_subcommands(&command.commands, 6);
        if !command.owner.is_empty() {
            Logger::with_theme(|theme| {
                println!(
                    "\n{}{}{}",
                    Logger::indent(Some(9)),
                    theme.description("Owned by: "),
                    Logger::cyan(&command.owner),
                );
            });
        }
    }

    pub fn log_external_subcommands(map: &HashMap<String, CommandDefinition>, indentation: i32) {
        for (name, command) in map {
            Logger::with_theme(|theme| {
                println!(
                    "{}{}: {}",
                    Logger::indent(Some(indentation)),
                    theme.sub_command(name),
                    theme.description(&command.description),
                );
            });
            Help::log_args(&command.args, Some(indentation + 3));
        }
    }

    pub fn log_internal_commands(internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        if internals.is_empty() {
            return;
        }
        let sorted_internals = Help::sort_internal(internals);
        Logger::space_around("Internal Commands:");
        for internal in sorted_internals {
            Help::log_internal_command(internal.get_definition());
            println!();
        }
    }

    pub fn log_root_commands(root_commands: &HashMap<String, CommandDefinition>) {
        if root_commands.is_empty() {
            return;
        }
        Logger::info("Project Level Commands:");
        Logger::with_surrounding_space(|| {
            let sorted_commands = Help::sort_root_commands(root_commands);
            for command in sorted_commands {
                Help::log_root_command(&command);
            }
        });
    }

    pub fn log_external_commands(externals: &HashMap<String, RepoKitCommand>) {
        if externals.is_empty() {
            return;
        }
        let sorted_externals = Help::sort_external(externals);
        Logger::info("Registered Commands:");
        println!();
        for external in sorted_externals {
            Help::log_external_command(external);
            println!();
        }
    }

    fn log_args(map: &Option<HashMap<String, String>>, indentation: Option<i32>) {
        Logger::with_theme(|theme| {
            if let Some(args) = map {
                for (name, description) in args {
                    println!(
                        "{}{} {}",
                        Logger::indent(Some(indentation.unwrap_or(6))),
                        theme.arg(name),
                        theme.description(description)
                    );
                }
            }
        });
    }

    fn sort_internal(
        commands: &HashMap<String, Box<dyn InternalExecutable>>,
    ) -> Vec<&Box<dyn InternalExecutable>> {
        let mut vector: Vec<&Box<dyn InternalExecutable>> = commands.values().collect();
        sort_slice_by_str_key(&mut vector, |x| &x.get_definition().name);
        vector
    }

    fn sort_external(commands: &HashMap<String, RepoKitCommand>) -> Vec<&RepoKitCommand> {
        let mut vector: Vec<&RepoKitCommand> = (commands).values().collect();
        sort_slice_by_str_key(&mut vector, |x| &x.name);
        vector
    }

    fn sort_root_commands(commands: &HashMap<String, CommandDefinition>) -> Vec<RootCommand> {
        let mut vector: Vec<&String> = (commands).keys().collect();
        sort_str_slice(&mut vector);
        vector
            .iter()
            .map(|&name| {
                RootCommand::from(
                    name,
                    commands
                        .get(name)
                        .expect("iteration is over known keys only"),
                )
            })
            .collect()
    }
}
