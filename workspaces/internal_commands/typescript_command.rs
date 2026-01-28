use std::path::{Path, PathBuf};

use serde_json::from_str;

use crate::{
    configuration::configuration::{DevKitCommand, DevKitConfig},
    executor::executor::Executor,
};

pub struct TypescriptCommand;

impl TypescriptCommand {
    pub fn parse_configuration(root: &String) -> DevKitConfig {
        let executable = TypescriptCommand::path_to_command("parse_configuration.ts");
        let stdout = Executor::exec(format!("npx tsx {executable} --root {root}").as_str());
        let DevKitConfig {
            project,
            workspaces,
            commands,
        } = from_str(stdout.as_str()).expect("Error parsing stdout");
        return DevKitConfig {
            project,
            workspaces,
            commands,
        };
    }

    pub fn parse_commands(path_list: Vec<String>) -> Vec<DevKitCommand> {
        let paths = path_list.join(",");
        let executable = TypescriptCommand::path_to_command("parse_commands.ts");
        let stdout = Executor::exec(format!("npx tsx {executable} --paths {paths}"));
        let commands: Vec<DevKitCommand> = serde_json::from_str(&stdout).expect("parse");
        return commands;
    }

    fn commands_dir() -> PathBuf {
        let file_path = file!();
        let dir = Path::new(file_path)
            .parent()
            .expect("Failed to get parent directory");
        return dir.join("../../src/commands");
    }

    fn path_to_command(command_file: &str) -> String {
        return TypescriptCommand::commands_dir()
            .join(command_file)
            .into_os_string()
            .into_string()
            .expect("Cannot construct path");
    }
}
