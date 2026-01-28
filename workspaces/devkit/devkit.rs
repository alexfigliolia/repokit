use ::futures::executor;
use std::{
    collections::HashMap,
    env::args,
    process::{self},
};

use crate::{
    configuration::configuration::{DevKitCommand, DevKitConfig},
    executables::intenal_executable::InternalExecutable,
    executor::executor::Executor,
    external_commands::external_commands::ExternalCommands,
    internal_commands::{
        help::Help, locate_command::LocateCommand, register_command::RegisterCommand,
    },
    logger::logger::Logger,
};

pub struct DevKit {
    root: String,
    configuration: DevKitConfig,
}

impl DevKit {
    pub fn new(root: String, configuration: DevKitConfig) -> DevKit {
        DevKit {
            root,
            configuration,
        }
    }

    pub fn invoke(&self) {
        let (command, args) = self.parse();
        let internals = self.internal_commands();
        if internals.contains_key(&command) {
            match internals.get(&command) {
                Some(command) => {
                    return command.run(args);
                }
                None => {}
            }
        }
        let externals = self.external_commands();
        ExternalCommands::validate(&internals, &externals);
        if externals.contains_key(&command) {
            if &args.len() <= &0 {
                Logger::info(
                    format!(
                        "Listing available commands for {}",
                        Logger::blue_bright(&command)
                    )
                    .as_str(),
                );
                return Help::external_command(externals.get(&command).expect("found"));
            }
            let sub_command = &args[0];
            let devkit = externals.get(&command).expect("Exists");
            if devkit.commands.contains_key(sub_command) {
                let script = devkit.commands.get(sub_command).expect("Exists");
                return Executor::with_stdio(format!(
                    "{}{}",
                    &script.command,
                    &args[1..].join(" ")
                ));
            }
            self.subcommand_not_found(&command, &sub_command);
            return Help::external_command(&devkit);
        }
        self.command_not_found(&command);
        return Help::list_all(&internals, &externals);
    }

    fn parse(&self) -> (String, Vec<String>) {
        let argv: Vec<String> = args().collect();
        if argv.len() < 2 {
            Help::list_all(&self.internal_commands(), &self.external_commands());
            process::exit(0);
        }
        let command = &argv[1];
        let args = &(&argv)[2..];
        return (command.clone(), args.to_vec());
    }

    fn internal_commands(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let commands: [Box<dyn InternalExecutable>; 2] = [
            Box::new(RegisterCommand::new(self.root.clone())),
            Box::new(LocateCommand::new(self.root.clone())),
        ];
        return HashMap::from(commands.map(|x| (x.get_definition().name.to_string(), x)));
    }

    fn external_commands(&self) -> HashMap<String, DevKitCommand> {
        let finder = ExternalCommands::new(self.root.clone());
        return executor::block_on(finder.find_all());
    }

    fn command_not_found(&self, command: &str) {
        Logger::info(
            format!(
                "I'm not aware of a command named {}",
                Logger::blue_bright(&command)
            )
            .as_str(),
        );
    }

    fn subcommand_not_found(&self, command: &str, sub_command: &str) {
        Logger::info(
            format!(
                "The command {} was not found on {}",
                Logger::blue_bright(sub_command),
                Logger::blue_bright(command)
            )
            .as_str(),
        );
        Logger::info(
            format!(
                "Here are the commands that belong to {}",
                Logger::blue_bright(command)
            )
            .as_str(),
        );
    }
}
