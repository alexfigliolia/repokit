use normalize_path::NormalizePath;
use std::{
    collections::HashMap,
    fs::{File, create_dir_all},
    io,
    path::{Path, PathBuf},
    process,
};

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    internal_filesystem::internal_filesystem::InternalFileSystem,
    logger::logger::Logger,
    repokit::interfaces::RepoKitConfig,
};

pub struct RegisterCommand {
    pub root: String,
    pub configuration: RepoKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl RegisterCommand {
    pub fn new(root: String, configuration: RepoKitConfig) -> RegisterCommand {
        RegisterCommand {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "register",
                description: "Creates new Repokit commands",
                args: HashMap::from([(
                    "<path>",
                    "A relative path to your preferred command location",
                )]),
            },
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
        let path = Path::new(&self.root).join(&path_arg).normalize();
        if !path.exists() {
            Logger::info(
                format!(
                    "Creating the path {} in your file system",
                    Logger::blue_bright(path_arg.as_str())
                )
                .as_str(),
            );
            create_dir_all(&path).expect("");
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
        let template_path = InternalFileSystem::resolve_template("command_template.ts");
        let mut source = File::open(template_path).expect("Template");
        let mut target = File::create(&command_path).expect("creating");
        io::copy(&mut source, &mut target).expect("writing");
        target.sync_all().expect("Flushing");
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
