use normalize_path::NormalizePath;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
};

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput, RepoKitScope,
        },
    },
    internal_commands::help::Help,
    internal_filesystem::{file_builder::FileBuilder, internal_filesystem::InternalFileSystem},
    logger::logger::Logger,
};

pub struct RegisterCommand {
    pub scope: RepoKitScope,
    pub definition: InternalExecutableDefinition,
}

impl RegisterCommand {
    pub fn new(scope: &RepoKitScope) -> RegisterCommand {
        RegisterCommand {
            scope: scope.clone(),
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
        let path = Path::new(&self.scope.root).join(&path_arg).normalize();
        if !path.exists() {
            Logger::info(
                format!(
                    "Creating the path {} in your file system",
                    Logger::blue_bright(path_arg.as_str())
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
                    Logger::blue_bright("Commands.ts")
                )
                .as_str(),
            );
            Logger::info(format!(
                "You can append additional commands to the existing {} instance or export another one",
                Logger::blue_bright("RepoKitCommand")
            ).as_str());
            process::exit(0);
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
        let mut source =
            InternalFileSystem::new(&self.scope.root).resolve_template("command_template.txt");
        let mut target = FileBuilder::create(&command_path, |_| Logger::file_create_error());
        FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
        Logger::info("Creating command file");
        Logger::info("Please fill out your command file located at:");
        Logger::log_file_path(command_path.to_str().expect("path"));
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
