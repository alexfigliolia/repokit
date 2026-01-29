use normalize_path::NormalizePath;
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{File, create_dir_all},
    io,
    path::{Path, PathBuf},
    process,
};

use crate::{
    devkit::interfaces::DevKitConfig,
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct RegisterCommand {
    pub root: String,
    pub configuration: DevKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl RegisterCommand {
    pub fn new(root: String, configuration: DevKitConfig) -> RegisterCommand {
        RegisterCommand {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "register-command",
                description: "Creates new Devkit commands",
                args: HashMap::from([(
                    "--path | -p",
                    "A relative path to your preferred command location",
                )]),
            },
        }
    }

    fn parse(&self, args: Vec<String>) -> PathBuf {
        use lexopt::prelude::*;
        let mut path = String::from("");
        let mut parser = lexopt::Parser::from_args(args);
        while let Ok(Some(arg)) = parser.next() {
            match arg {
                Short('p') | Long("path") => {
                    path = parser
                        .value()
                        .unwrap_or(OsString::from(""))
                        .into_string()
                        .expect("Derp");
                }
                _ => {}
            }
        }
        self.validate_path(path)
    }

    fn validate_path(&self, path_arg: String) -> PathBuf {
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
                Logger::blue_bright("DevKitCommand")
            ).as_str());
            process::exit(0);
        }
        command_path.clone()
    }

    fn exit_on_missing_path() {
        Logger::exit_with_error(
                format!(
                    "Please specify a path to a directory relative to the root of your repository using the {} argument",
                    Logger::blue_bright("--path | -p")
                )
                .as_str(),
            );
    }

    pub fn template_path() -> PathBuf {
        let file_path = file!();
        let dir = Path::new(file_path)
            .parent()
            .expect("Failed to get parent directory");
        dir.join("command_template.ts")
    }
}

impl InternalExecutable for RegisterCommand {
    fn run(&self, args: Vec<String>) {
        Logger::info("Registering a new command");
        let command_path = self.parse(args);
        let mut source = File::open(RegisterCommand::template_path()).expect("Template");
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
