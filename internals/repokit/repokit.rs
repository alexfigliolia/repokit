use std::{
    collections::HashMap,
    env::args,
    path::Path,
    process::{self},
};

use crate::{
    executables::{
        intenal_executable::InternalExecutable, internal_executable_definition::RepoKitScope,
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::interfaces::{RepoKitCommand, RepoKitConfig},
    validations::command_validations::CommandValidations,
};

pub struct RepoKit {
    pub scope: RepoKitScope,
}

impl RepoKit {
    pub fn new(root: String, configuration: RepoKitConfig) -> RepoKit {
        Logger::set_name(&configuration.project);
        RepoKit {
            scope: RepoKitScope {
                root,
                configuration,
            },
        }
    }

    pub fn invoke(&self) {
        let (command, args) = self.parse();
        let validator = CommandValidations::from(self);
        let internals = validator.collect_and_validate_internals();
        if internals.contains_key(&command) {
            let interface = internals.get(&command).expect("Unknown command");
            return interface.run(args, &internals);
        }
        if self.scope.configuration.commands.contains_key(&command) {
            let root_script = self
                .scope
                .configuration
                .commands
                .get(&command)
                .expect("Unknown command");
            return Executor::with_stdio(
                format!("{} {}", root_script.command, &args.join(" ")),
                |cmd| cmd.current_dir(Path::new(&self.scope.root)),
            );
        }
        let externals = validator.collect_and_validate_externals();
        CommandValidations::detect_collisions_between_internals_and_externals(
            &internals, &externals,
        );
        if externals.contains_key(&command) {
            let interface = externals.get(&command).expect("Unknown command");
            if args.is_empty() {
                return self.log_external_command(interface);
            }
            let sub_command = &args[0];
            if interface.commands.contains_key(sub_command) {
                let script = interface.commands.get(sub_command).expect("Unknown script");
                let working_dir = Path::new(&interface.location)
                    .parent()
                    .expect("Working directory not found");
                return Executor::with_stdio(
                    format!("{} {}", &script.command, &args[1..].join(" ")),
                    |cmd| cmd.current_dir(working_dir),
                );
            }
            return self.subcommand_not_found(interface, sub_command);
        }
        self.command_not_found(&command, &internals, &externals)
    }

    fn parse(&self) -> (String, Vec<String>) {
        let argv: Vec<String> = args().collect();
        if argv.len() < 2 {
            let (internals, externals) = self.collect_and_validate();
            Help::list_all(&self.scope.configuration.commands, &internals, &externals);
            process::exit(0);
        }
        let command = &argv[1];
        let args = &(&argv)[2..];
        (command.clone(), args.to_vec())
    }

    fn collect_and_validate(
        &self,
    ) -> (
        HashMap<String, Box<dyn InternalExecutable>>,
        HashMap<String, RepoKitCommand>,
    ) {
        let validator = CommandValidations::from(self);
        let internals = validator.collect_and_validate_internals();
        let externals = validator.collect_and_validate_externals();
        CommandValidations::detect_collisions_between_internals_and_externals(
            &internals, &externals,
        );
        (internals, externals)
    }

    fn command_not_found(
        &self,
        command: &str,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, RepoKitCommand>,
    ) {
        Help::list_all(&self.scope.configuration.commands, internals, externals);
        Logger::info(
            format!(
                "I'm not aware of a command named {}",
                Logger::blue_bright(command)
            )
            .as_str(),
        );
    }

    fn subcommand_not_found(&self, command: &RepoKitCommand, sub_command: &str) {
        Logger::info(
            format!(
                "The command {} was not found on {}",
                Logger::blue_bright(sub_command),
                Logger::blue_bright(&command.name)
            )
            .as_str(),
        );
        Logger::info(
            format!(
                "Here are the commands that belong to {}",
                Logger::blue_bright(&command.name)
            )
            .as_str(),
        );
        Help::log_external_subcommands(&command.commands, 3);
    }

    fn log_external_command(&self, command: &RepoKitCommand) {
        Logger::info(
            format!(
                "Listing available commands for {}\n",
                Logger::blue(&command.name)
            )
            .as_str(),
        );
        Help::log_external_subcommands(&command.commands, 3);
        println!();
    }
}
