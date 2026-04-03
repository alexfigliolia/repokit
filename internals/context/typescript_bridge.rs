use std::{path::Path, process::exit, sync::MutexGuard};

use serde_json::{Value, from_str};

use crate::{
    configuration::configuration::Configuration,
    context::file_system::FileSystem,
    executor::executor::Executor,
    post_processing::post_processor::PostProcessor,
    repokit::{
        repokit_command::RepoKitCommand, repokit_config::RepoKitConfig,
        repokit_construct_validator::RepoKitConstructValidator,
    },
};

#[derive(Clone)]
pub struct TypeScriptBridge {
    files: FileSystem,
}

impl TypeScriptBridge {
    pub fn new(file_system: &FileSystem) -> TypeScriptBridge {
        TypeScriptBridge {
            files: file_system.to_owned(),
        }
    }

    pub fn parse_configuration(&self) -> RepoKitConfig {
        let executable = self.files.resolve_command("parse_configuration");
        let stdout = self.execute(format!("{executable} --root {}", &self.files.root).as_str());
        if stdout.is_empty() {
            Configuration::create(&self.files);
        }
        let result: Result<Value, serde_json::Error> = from_str(stdout.as_str());
        match result {
            Ok(config) => RepoKitConfig::from_input(&self.files.root, config),
            Err(_) => {
                RepoKitConfig::on_parsing_error(&self.files.root, Value::Null);
                PostProcessor::get().flush();
                exit(0)
            }
        }
    }

    pub fn parse_commands(&self, path_list: &MutexGuard<Vec<String>>) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let executable = self.files.resolve_command("parse_commands");
        let stdout = self
            .execute(format!("{executable} --paths {paths} --root {}", self.files.root).as_str());
        let result: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&stdout);
        match result {
            Ok(commands) => RepoKitCommand::from_input(&self.files.root, commands),
            Err(_) => {
                RepoKitCommand::on_parsing_error(&self.files.root, Value::Null);
                PostProcessor::get().flush();
                exit(0);
            }
        }
    }

    fn execute(&self, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(Path::new(&self.files.root))
        })
    }
}
