use std::path::{Path, PathBuf};

use normalize_path::NormalizePath;
use serde_json::from_str;

use crate::{
    configuration::configuration::Configuration,
    devkit::interfaces::{DevKitCommand, DevKitConfig},
    executor::executor::Executor,
};

pub struct TypescriptCommand {
    root: String,
}

impl TypescriptCommand {
    pub fn new(root: String) -> TypescriptCommand {
        TypescriptCommand { root }
    }

    pub fn parse_configuration(&self) -> DevKitConfig {
        let executable = self.path_to_command("parse_configuration.ts");
        let stdout = self.execute(format!("{executable} --root {}", &self.root).as_str());
        if stdout.is_empty() {
            Configuration::create(&self.root);
        }
        let DevKitConfig { project, commands } =
            from_str(stdout.as_str()).expect("Error parsing stdout");
        DevKitConfig { project, commands }
    }

    pub fn parse_commands(&self, path_list: Vec<String>) -> Vec<DevKitCommand> {
        let paths = path_list.join(",");
        let executable = self.path_to_command("parse_commands.ts");
        let stdout =
            self.execute(format!("{executable} --paths {paths} --root {}", self.root).as_str());
        let commands: Vec<DevKitCommand> = serde_json::from_str(&stdout).expect("parse");
        commands
    }

    fn commands_dir(&self) -> PathBuf {
        let file_path = file!();
        let dir = Path::new(file_path)
            .parent()
            .expect("Failed to get parent directory");
        let resolved = Path::new(&self.root).join(dir);
        resolved.join("../../src/commands").normalize()
    }

    fn path_to_command(&self, command_file: &str) -> String {
        self.commands_dir()
            .join(command_file)
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }

    fn execute(&self, args: &str) -> String {
        Executor::exec(format!("npx tsx {}", args), |cmd| {
            cmd.current_dir(Path::new(&self.root))
        })
    }
}
