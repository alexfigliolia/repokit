use std::collections::HashMap;

use crate::{
    devkit::interfaces::{Command, DevKitCommand, ParsedCommand},
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    logger::logger::Logger,
};

pub struct Help;

impl Help {
    pub fn list_all(
        root_commands: &HashMap<String, Command>,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        Help::log_internal_commands(internals);
        Help::log_root_commands(root_commands);
        Help::log_external_commands(externals);
    }

    pub fn log_internal_command(command: &InternalExecutableDefinition) {
        println!(
            "{}{} {}",
            Logger::indent(Some(3)),
            Logger::blue_bright(command.name),
            Logger::gray(command.description),
        );
        Help::print_args(&command.args);
    }

    pub fn log_root_command(command: &ParsedCommand) {
        println!(
            "{}{} {}",
            Logger::indent(Some(3)),
            Logger::blue_bright(&command.name),
            Logger::gray(&command.description),
        );
    }

    pub fn log_external_command(command: &DevKitCommand) {
        println!(
            "{}{} {}",
            Logger::indent(Some(3)),
            Logger::blue_bright(&command.name),
            Logger::gray(&command.description),
        );
        Help::print_commands(&command.commands, Some(6));
    }

    pub fn print_commands(map: &HashMap<String, Command>, indentation: Option<i32>) {
        for (name, command) in map {
            println!(
                "{}",
                format!(
                    "{}{}{}",
                    Logger::indent(indentation),
                    Logger::green(format!("{}: ", name).as_str()),
                    Logger::gray(&command.description),
                )
            );
        }
    }

    fn log_internal_commands(internals: &HashMap<String, Box<dyn InternalExecutable>>) {
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

    fn log_root_commands(root_commands: &HashMap<String, Command>) {
        if root_commands.is_empty() {
            return;
        }
        let sorted_commands = Help::sort_root_commands(root_commands);
        Logger::info("Project Level Commands:");
        println!();
        for command in sorted_commands {
            Help::log_root_command(&command);
        }
        println!();
    }

    fn log_external_commands(externals: &HashMap<String, DevKitCommand>) {
        if externals.is_empty() {
            return;
        }
        let sorted_externals = Help::sort_external(externals);
        Logger::info("Package Level Commands:");
        println!();
        for external in sorted_externals {
            Help::log_external_command(external);
            println!();
        }
    }

    fn print_args(map: &HashMap<&'static str, &'static str>) {
        for (name, description) in map {
            println!(
                "{}",
                format!(
                    "{}{}{}",
                    Logger::indent(Some(6)),
                    Logger::green(format!("{}: ", name).as_str()),
                    Logger::gray(description),
                )
            );
        }
    }

    fn sort_internal(
        commands: &HashMap<String, Box<dyn InternalExecutable>>,
    ) -> Vec<&Box<dyn InternalExecutable>> {
        let mut vector: Vec<&Box<dyn InternalExecutable>> = commands.values().collect();
        vector.sort_by_key(|x| &x.get_definition().name);
        vector
    }

    fn sort_external(commands: &HashMap<String, DevKitCommand>) -> Vec<&DevKitCommand> {
        let mut vector: Vec<&DevKitCommand> = (commands).values().collect();
        vector.sort_by_key(|x| &x.name);
        vector
    }

    fn sort_root_commands(commands: &HashMap<String, Command>) -> Vec<ParsedCommand> {
        let mut vector: Vec<&String> = (commands).keys().collect();
        vector.sort();
        vector
            .iter()
            .map(|&name| ParsedCommand::from(name, commands.get(name).expect("known keys only")))
            .collect()
    }
}
