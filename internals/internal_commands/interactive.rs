use std::{collections::HashMap, path::Path};

use alphanumeric_sort::sort_str_slice;
use colored::Colorize;
use inquire::{InquireError, Select, Text};

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
    prompts::{bold_option::BoldOption, command_scope::CommandScope, inquire_theme::InquireTheme},
    repokit::repokit_runtime::RepoKitRuntime,
    validations::command_validations::CommandValidations,
};

pub struct Interactive {
    pub definition: InternalExecutableDefinition,
}

impl Interactive {
    pub fn new() -> Interactive {
        Interactive {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "interactive",
                description: "Opens your repokit command library in interactive mode",
                args: [],
            }),
        }
    }

    pub fn prompt_scope(&self) -> Result<CommandScope, InquireError> {
        CommandScope::select(&Logger::with_info_prefix("Is the tool you're looking for one of repokit's internals, a registered command, or one defined in your config file?\n"))
            .with_render_config(InquireTheme::create())
            .prompt()
    }

    pub fn prompt_arguments(&self, command_name: &str) -> Result<String, InquireError> {
        let result = Text::new(&Logger::with_info_prefix(&format!(
            "Please specify any command arguments you wish to execute {} with\n",
            Logger::with_theme(|theme| theme.highlight(command_name))
        )))
        .with_formatter(&|v| format!("{}", v.to_owned().bold()))
        .with_render_config(InquireTheme::create())
        .prompt();
        println!();
        result
    }

    pub fn to_options(
        &self,
        scope: CommandScope,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
    ) -> Vec<String> {
        let mut options: Vec<String> = Vec::new();
        match scope {
            CommandScope::Internal => {
                for key in internals.keys() {
                    options.push(key.to_owned())
                }
            }
            CommandScope::Root => {
                let root_commands = RepoKitRuntime::with_runtime(|runtime| {
                    runtime.configuration.commands.to_owned()
                });
                if root_commands.is_empty() {
                    Logger::info("There are no commands registered in your configuration file");
                    Logger::exit_with_info(&format!(
                        "You can add root level commands using your {}'s {} property",
                        Logger::with_theme(|theme| theme.highlight("RepoKitConfig")),
                        Logger::with_theme(|theme| theme.highlight("commands")),
                    ));
                }
                for key in root_commands.keys() {
                    options.push(key.to_owned())
                }
            }
            CommandScope::Registered => {
                let externals = CommandValidations::collect_and_validate_externals();
                if externals.is_empty() {
                    Logger::exit_with_error(
                        "You do not have any registered commands in this repository",
                    );
                }
                for key in externals.keys() {
                    options.push(key.to_owned())
                }
            }
        }
        options
    }

    pub fn execute_internal_command(
        &self,
        command: &str,
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
    ) {
        if let Some(definition) = internals.get(command) {
            self.define(command);
            definition.help();
            println!();
            let mut arguments = "".to_owned();
            if let Some(args) = &definition.get_definition().args
                && !args.is_empty()
                && let Ok(args) = self.prompt_arguments(&definition.get_definition().name)
            {
                arguments = args;
            }
            let args_vector: Vec<String> = arguments.split(" ").map(|arg| arg.to_owned()).collect();
            definition.run(args_vector, internals);
            panic!();
        }
    }

    pub fn execute_root_command(&self, command: &str) {
        RepoKitRuntime::with_runtime(|runtime| {
            if let Some(definition) = runtime.configuration.commands.get(command)
                && let Ok(args) = self.prompt_arguments(command)
            {
                Executor::with_stdio(format!("{} {}", definition.command, args), |cmd| {
                    cmd.current_dir(&runtime.typescript_library.install_path)
                });
                panic!();
            }
        });
    }

    pub fn execute_registered_command(&self, command: &str) {
        let externals = CommandValidations::collect_and_validate_externals();
        if let Some(definition) = externals.get(command) {
            self.define(command);
            Help::log_external_command(definition);
            println!();
            let mut options: Vec<String> = definition
                .commands
                .keys()
                .map(|key| key.to_owned())
                .collect();
            sort_str_slice(&mut options);
            if let Ok(sub_command) = Select::new(
                &Logger::with_info_prefix(&format!(
                    "Please select one of the {} commands\n",
                    definition.name
                )),
                self.to_bold(&options),
            )
            .with_render_config(InquireTheme::create())
            .prompt()
                && let Some(script) = definition.commands.get(sub_command.get())
                && let Ok(args) = self.prompt_arguments(command)
            {
                let executable = format!("{} {}", script.command, args);
                if let Some(working_dir) = Path::new(&definition.location).parent() {
                    Executor::with_stdio(executable, |cmd| cmd.current_dir(working_dir));
                    panic!();
                }
            }
        }
    }

    pub fn define(&self, command: &str) {
        Logger::info(&format!(
            "Listing the definition for {}\n",
            Logger::with_theme(|theme| theme.highlight(command))
        ));
    }

    pub fn to_bold(&self, options: &[String]) -> Vec<BoldOption> {
        options
            .iter()
            .map(|opt| BoldOption(opt.to_owned()))
            .collect()
    }
}

impl InternalExecutable for Interactive {
    fn run(&self, _args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        if let Ok(desired_scope) = self.prompt_scope() {
            let mut options = self.to_options(desired_scope, internals);
            sort_str_slice(&mut options);
            let result: Result<BoldOption, InquireError> = Select::new(
                &Logger::with_info_prefix("Please select a command from the list of options\n"),
                self.to_bold(&options),
            )
            .with_render_config(InquireTheme::create())
            .prompt();
            println!();
            if let Ok(command) = result {
                match desired_scope {
                    CommandScope::Internal => {
                        self.execute_internal_command(command.get(), internals);
                    }
                    CommandScope::Root => {
                        self.execute_root_command(command.get());
                    }
                    CommandScope::Registered => {
                        self.execute_registered_command(command.get());
                    }
                }
            }
        }
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
