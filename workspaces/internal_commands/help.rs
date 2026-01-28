use std::collections::HashMap;

use crate::{
    configuration::configuration::{Command, DevKitCommand},
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    logger::logger::Logger,
};

pub struct Help;

impl Help {
    pub fn list_all(
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        let sorted_internals = Help::sort_internal(&internals);
        let sorted_externals = Help::sort_external(&externals);
        Logger::space_around("Internal Commands:");
        for internal in sorted_internals {
            Help::internal_command(&internal.get_definition());
            println!("");
        }
        Logger::info("Registered Commands:");
        println!("");
        for external in sorted_externals {
            Help::external_command(&external);
            println!("");
        }
    }

    pub fn internal_command(command: &InternalExecutableDefinition) {
        println!(
            "{}{}",
            Logger::indent(Some(3)),
            Logger::blue_bright(&command.name)
        );
        Help::print_args(&command.args);
    }

    pub fn external_command(command: &DevKitCommand) {
        println!(
            "{}{}",
            Logger::indent(Some(3)),
            Logger::blue_bright(&command.name)
        );
        Help::print_commands(&command.commands);
    }

    fn print_args(map: &HashMap<&'static str, &'static str>) {
        for (name, description) in map {
            println!(
                "{}",
                format!(
                    "{}{}{}",
                    Logger::indent(Some(6)),
                    Logger::green(format!("{}: ", name).as_str()),
                    description,
                )
            );
        }
    }

    fn print_commands(map: &HashMap<String, Command>) {
        for (name, command) in map {
            println!(
                "{}",
                format!(
                    "{}{}{}",
                    Logger::indent(Some(6)),
                    Logger::green(format!("{}: ", name).as_str()),
                    command.description,
                )
            );
        }
    }

    fn sort_internal(
        commands: &HashMap<String, Box<dyn InternalExecutable>>,
    ) -> Vec<&Box<dyn InternalExecutable>> {
        let mut vector: Vec<&Box<dyn InternalExecutable>> = commands.values().collect();
        vector.sort_by_key(|x| &x.get_definition().name);
        return vector;
    }

    fn sort_external(commands: &HashMap<String, DevKitCommand>) -> Vec<&DevKitCommand> {
        let mut vector: Vec<&DevKitCommand> = (commands).values().collect();
        vector.sort_by_key(|x| &x.name);
        return vector;
    }
}
