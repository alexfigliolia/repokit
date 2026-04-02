use std::{path::Path, process::exit, sync::MutexGuard};

use serde_json::{Value, from_str};

use crate::{
    configuration::configuration::Configuration,
    executor::executor::Executor,
    internal_filesystem::internal_filesystem::InternalFileSystem,
    post_processing::post_processor::PostProcessor,
    repokit::{
        repokit_command::RepoKitCommand, repokit_config::RepoKitConfig,
        repokit_construct_validator::RepoKitConstructValidator,
    },
};

pub struct TypescriptCommand {
    root: String,
}

impl TypescriptCommand {
    pub fn new(root: &str) -> TypescriptCommand {
        TypescriptCommand {
            root: root.to_owned(),
        }
    }

    pub fn parse_configuration(&self) -> RepoKitConfig {
        let executable = InternalFileSystem::new(&self.root).resolve_command("parse_configuration");
        let stdout = self.execute(format!("{executable} --root {}", &self.root).as_str());
        if stdout.is_empty() {
            Configuration::create(&self.root);
        }
        let result: Result<Value, serde_json::Error> = from_str(stdout.as_str());
        match result {
            Ok(config) => RepoKitConfig::from_input(&self.root, config),
            Err(_) => {
                RepoKitConfig::on_parsing_error(&self.root, Value::Null);
                PostProcessor::get().flush();
                exit(0)
            }
        }
    }

    pub fn parse_commands(&self, path_list: &MutexGuard<Vec<String>>) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let executable = InternalFileSystem::new(&self.root).resolve_command("parse_commands");
        let stdout =
            self.execute(format!("{executable} --paths {paths} --root {}", self.root).as_str());
        let result: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&stdout);
        match result {
            Ok(commands) => RepoKitCommand::from_input(&self.root, commands),
            Err(_) => {
                RepoKitCommand::on_parsing_error(&self.root, Value::Null);
                PostProcessor::get().flush();
                exit(0);
            }
        }
    }

    fn execute(&self, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(Path::new(&self.root))
        })
    }
}
