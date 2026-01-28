use ::futures::executor;
use std::collections::HashMap;

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    external_commands::external_commands::ExternalCommands,
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct LocateCommand {
    pub root: String,
    pub definition: InternalExecutableDefinition,
}

impl LocateCommand {
    pub fn new(root: String) -> LocateCommand {
        LocateCommand {
            root,
            definition: InternalExecutableDefinition {
                name: "locate-command",
                description: "Locates the command definition for a registered command",
                args: HashMap::from([("<name>", "The name of a registered command")]),
            },
        }
    }
}

impl InternalExecutable for LocateCommand {
    fn run(&self, args: Vec<String>) {
        if args.len() == 0 {
            Logger::exitWithInfo("Please specify a command to locate");
        }
        let command = &args[0];
        Logger::info(format!("Locating a command named {}", Logger::blue_bright(command)).as_str());
        let finder = ExternalCommands::new(self.root.clone());
        let commands = executor::block_on(finder.find_all());
        if commands.contains_key(command) {
            let interface = commands.get(command).expect("exists");
            return println!(
                "\n{}{}\n",
                Logger::indent(Some(3)),
                Logger::blue(&interface.location)
            );
        }
        Logger::exitWithError(
            format!(
                "I could not find a command named {}",
                Logger::blue_bright(command)
            )
            .as_str(),
        );
    }

    fn help(&self) {
        Help::internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        return &self.definition;
    }
}
