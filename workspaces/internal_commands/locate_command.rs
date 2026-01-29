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
                description: "Locates command definitions",
                args: HashMap::from([("<name>", "The name of a registered command")]),
            },
        }
    }
}

impl InternalExecutable for LocateCommand {
    fn run(&self, args: Vec<String>) {
        if args.is_empty() {
            Logger::exit_with_info("Please specify a command to locate");
        }
        let command = &args[0];
        Logger::info(format!("Locating a command named {}", Logger::blue_bright(command)).as_str());
        let finder = ExternalCommands::new(self.root.clone());
        let commands = executor::block_on(finder.find_all());
        if commands.contains_key(command) {
            let interface = commands.get(command).expect("exists");
            return Logger::log_file_path(&interface.location);
        }
        Logger::exit_with_error(
            format!(
                "I could not find a command named {}",
                Logger::blue_bright(command)
            )
            .as_str(),
        );
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
