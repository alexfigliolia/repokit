use ::futures::executor;
use std::{
    collections::HashMap,
    env::args,
    process::{self},
};

use crate::{
    devkit::interfaces::{DevKitCommand, DevKitConfig},
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
        Logger::set_name(&configuration.project);
        DevKit {
            root,
            configuration,
        }
    }

    pub fn invoke(&self) {
        let (command, args) = self.parse();
        let internals = self.internal_commands();
        if internals.contains_key(&command) {
            let interface = internals.get(&command).expect("exists");
            return interface.run(args);
        }
        let externals = self.external_commands();
        ExternalCommands::validate(&internals, &externals);
        if externals.contains_key(&command) {
            let interface = externals.get(&command).expect("exists");
            if args.len() <= 0 {
                return self.log_external_command(interface);
            }
            let sub_command = &args[0];
            if interface.commands.contains_key(sub_command) {
                let script = interface.commands.get(sub_command).expect("exists");
                return Executor::with_stdio(format!(
                    "{}{}",
                    &script.command,
                    &args[1..].join(" ")
                ));
            }
            return self.subcommand_not_found(interface, sub_command);
        }
        self.command_not_found(&command, &internals, &externals)
    }

    fn parse(&self) -> (String, Vec<String>) {
        let argv: Vec<String> = args().collect();
        if argv.len() < 2 {
            Help::list_all(&self.internal_commands(), &self.external_commands());
            process::exit(0);
        }
        let command = &argv[1];
        let args = &(&argv)[2..];
        (command.clone(), args.to_vec())
    }

    fn internal_commands(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let commands: [Box<dyn InternalExecutable>; 2] = [
            Box::new(LocateCommand::new(self.root.clone())),
            Box::new(RegisterCommand::new(self.root.clone())),
        ];
        HashMap::from(commands.map(|x| (x.get_definition().name.to_string(), x)))
    }

    fn external_commands(&self) -> HashMap<String, DevKitCommand> {
        let finder = ExternalCommands::new(self.root.clone());
        executor::block_on(finder.find_all())
    }

    fn command_not_found(
        &self,
        command: &str,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        Logger::info(
            format!(
                "I'm not aware of a command named {}",
                Logger::cyan_bright(command)
            )
            .as_str(),
        );
        Help::list_all(internals, externals);
    }

    fn subcommand_not_found(&self, command: &DevKitCommand, sub_command: &str) {
        Logger::info(
            format!(
                "The command {} was not found on {}",
                Logger::cyan_bright(sub_command),
                Logger::cyan_bright(&command.name)
            )
            .as_str(),
        );
        Logger::info(
            format!(
                "Here are the commands that belong to {}",
                Logger::cyan_bright(&command.name)
            )
            .as_str(),
        );
        Help::print_commands(&command.commands, Some(3));
    }

    fn log_external_command(&self, command: &DevKitCommand) {
        Logger::info(
            format!(
                "Listing available commands for {}",
                Logger::cyan_bright(&command.name)
            )
            .as_str(),
        );
        Help::print_commands(&command.commands, Some(3))
    }
}
