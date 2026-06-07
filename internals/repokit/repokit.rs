use std::{collections::HashMap, env::args, path::Path};

use crate::{
    executables::internal_executable::InternalExecutable,
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::{repokit_command::RepoKitCommand, repokit_runtime::RepoKitRuntime},
    validations::command_validations::CommandValidations,
};

pub struct RepoKit {}

impl RepoKit {
    pub fn new() -> RepoKit {
        Logger::initialize();
        RepoKit {}
    }

    pub fn invoke(&self) {
        let (command, args) = self.parse();
        let internals = CommandValidations::collect_and_validate_internals();
        if let Some(internal_command) = internals.get(&command) {
            return internal_command.run(args, &internals);
        }
        RepoKitRuntime::with_runtime(|runtime| {
            if let Some(root_script) = runtime.configuration.commands.get(&command) {
                Executor::with_stdio(
                    format!("{} {}", root_script.command, &args.join(" ")),
                    |cmd| cmd.current_dir(&runtime.typescript_library.install_path),
                );
                panic!();
            }
        });
        let externals = CommandValidations::collect_and_validate_externals();
        CommandValidations::detect_collisions_between_internals_and_externals(
            &internals, &externals,
        );
        if let Some(interface) = externals.get(&command) {
            if args.is_empty() {
                return self.log_external_command(interface);
            }
            let sub_command = &args[0];
            if let Some(script) = interface.commands.get(sub_command) {
                let executable = format!("{} {}", &script.command, &args[1..].join(" "));
                if let Some(working_dir) = Path::new(&interface.location).parent() {
                    Executor::with_stdio(executable, |cmd| cmd.current_dir(working_dir));
                    return;
                }
                return self.working_directory_not_found(interface, &executable);
            }
            return self.subcommand_not_found(interface, sub_command);
        }
        self.command_not_found(&command, &internals, &externals)
    }

    fn parse(&self) -> (String, Vec<String>) {
        let argv: Vec<String> = args().collect();
        if argv.len() < 2 {
            let (internals, externals) = self.collect_and_validate();
            Help::list_all(&internals, &externals);
            panic!();
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
        let internals = CommandValidations::collect_and_validate_internals();
        let externals = CommandValidations::collect_and_validate_externals();
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
        Help::list_all(internals, externals);
        Logger::info(
            format!(
                "I'm not aware of a command named {}",
                Logger::with_theme(|theme| theme.highlight(command))
            )
            .as_str(),
        );
    }

    fn subcommand_not_found(&self, command: &RepoKitCommand, sub_command: &str) {
        Logger::info(
            format!(
                "The command {} was not found on {}",
                Logger::with_theme(|theme| theme.highlight(sub_command)),
                Logger::with_theme(|theme| theme.highlight(&command.name))
            )
            .as_str(),
        );
        Logger::info(
            format!(
                "Here are the commands that belong to {}",
                Logger::with_theme(|theme| theme.highlight(&command.name))
            )
            .as_str(),
        );
        Help::log_external_subcommands(&command.commands, 3);
    }

    fn log_external_command(&self, command: &RepoKitCommand) {
        Logger::info(
            format!(
                "Listing available commands for {}\n",
                Logger::with_theme(|theme| theme.command(&command.name))
            )
            .as_str(),
        );
        Help::log_external_subcommands(&command.commands, 3);
        println!();
    }

    fn working_directory_not_found(&self, interface: &RepoKitCommand, executable: &str) {
        Logger::info("I was unable to determine the working directory for this command");
        Logger::info(
            format!(
                "This typically indicates a bug within {}",
                Logger::with_theme(|theme| theme.highlight("Repokit"))
            )
            .as_str(),
        );
        Logger::info("Please file an issue at");
        Logger::log_issue_link();
        Logger::info("To run this command from your terminal, you can run:");
        Logger::log_file_path(executable);
        Logger::info("From the parent directory of");
        Logger::log_file_path(&interface.location);
        panic!();
    }
}
