use normalize_path::NormalizePath;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
    typescript_library::typescript_templates::TypeScriptTemplate,
};

pub struct RegisterCommand {
    pub definition: InternalExecutableDefinition,
}

impl RegisterCommand {
    pub fn new() -> RegisterCommand {
        RegisterCommand {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "register",
                description: "Creates new Repokit commands",
                args: [(
                    "<path>",
                    "A relative path to your preferred command location",
                )],
            }),
        }
    }

    fn validate_path(&self, args: Vec<String>) -> PathBuf {
        if args.is_empty() {
            RegisterCommand::exit_on_missing_path();
        }
        let path_arg = args[0].clone();
        if path_arg.is_empty() {
            RegisterCommand::exit_on_missing_path();
        }
        let path = RepoKitRuntime::with_runtime(|runtime| {
            runtime
                .typescript_library
                .install_path
                .join(&path_arg)
                .normalize()
        });
        if !path.exists() {
            Logger::info(
                format!(
                    "Creating the path {} in your file system",
                    Logger::with_theme(|theme| theme.highlight(path_arg.as_str()))
                )
                .as_str(),
            );
            FileBuilder::create_dir_all(&path, |_| Logger::file_directory_error());
        }
        if !path.is_dir() {
            RegisterCommand::exit_on_missing_path();
        }
        let command_path = &path.join("Commands.ts");
        if command_path.exists() {
            Logger::error(
                format!(
                    "A {} file already exists in this directory",
                    Logger::with_theme(|theme| theme.highlight("Commands.ts"))
                )
                .as_str(),
            );
            Logger::exit_with_info(format!(
                "You can append additional commands to the existing {} instance or export another one",
                Logger::with_theme(|theme| theme.highlight("RepoKitCommand"))
            ).as_str());
        }
        command_path.clone()
    }

    fn exit_on_missing_path() {
        Logger::exit_with_error(
            "Please specify a path to a directory relative to the root of your repository",
        );
    }
}

impl InternalExecutable for RegisterCommand {
    fn run(&self, args: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Registering a new command");
        let command_path = self.validate_path(args);
        let mut source = RepoKitRuntime::with_runtime(|runtime| {
            runtime
                .typescript_library
                .resolve_template(TypeScriptTemplate::CommandTemplate)
        });
        let mut target = FileBuilder::create(&command_path, |_| Logger::file_create_error());
        FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
        Logger::info("Creating command file");
        Logger::info("Please fill out your command file located at:");
        Logger::log_file_path(command_path.to_str().expect("path"));
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
