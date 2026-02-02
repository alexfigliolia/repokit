use std::path::Path;

use serde_json::from_str;

use crate::{
    configuration::configuration::Configuration,
    executor::executor::Executor,
    internal_filesystem::internal_filesystem::InternalFileSystem,
    repokit::interfaces::{RepoKitCommand, RepoKitConfig},
};

pub struct TypescriptCommand {
    root: String,
}

impl TypescriptCommand {
    pub fn new(root: String) -> TypescriptCommand {
        TypescriptCommand { root }
    }

    pub fn parse_configuration(&self) -> RepoKitConfig {
        let executable = InternalFileSystem::resolve_command("parse_configuration.ts");
        let stdout = self.execute(format!("{executable} --root {}", &self.root).as_str());
        if stdout.is_empty() {
            Configuration::create(&self.root);
        }
        let RepoKitConfig { project, commands } =
            from_str(stdout.as_str()).expect("Error parsing stdout");
        RepoKitConfig { project, commands }
    }

    pub fn parse_commands(&self, path_list: Vec<String>) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let executable = InternalFileSystem::resolve_command("parse_commands.ts");
        let stdout =
            self.execute(format!("{executable} --paths {paths} --root {}", self.root).as_str());
        let commands: Vec<RepoKitCommand> = serde_json::from_str(&stdout).expect("parse");
        commands
    }

    fn execute(&self, args: &str) -> String {
        Executor::exec(format!("npx tsx {args}"), |cmd| {
            cmd.current_dir(Path::new(&self.root))
        })
    }
}
