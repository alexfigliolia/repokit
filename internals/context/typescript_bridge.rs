use std::{path::Path, process::exit, sync::MutexGuard};

use serde_json::{Value, from_str};

use crate::{
    context::{file_system::FileSystem, node_scope::NodeScope},
    executor::executor::Executor,
    post_processing::post_processor::PostProcessor,
    repokit::{repokit_command::RepoKitCommand, repokit_config::RepoKitConfig},
};

pub struct TypeScriptBridge;

impl TypeScriptBridge {
    pub fn parse_configuration(files: &FileSystem, node: &mut NodeScope) -> RepoKitConfig {
        let executable = files.resolve_command("parse_configuration");
        let stdout = TypeScriptBridge::execute_with_node(
            &files.git_root,
            format!("{executable} --root {}", &files.git_root).as_str(),
        );
        if stdout.is_empty() {
            RepoKitConfig::create(files);
        }
        let result: Result<Value, serde_json::Error> = from_str(stdout.as_str());
        match result {
            Ok(config) => RepoKitConfig::from_input(&files.git_root, node, config),
            Err(_) => {
                RepoKitConfig::on_parsing_error(&files.git_root, node, Value::Null);
                PostProcessor::get().flush();
                exit(0)
            }
        }
    }

    pub fn parse_commands(
        files: &FileSystem,
        node: &mut NodeScope,
        path_list: &MutexGuard<Vec<String>>,
    ) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let executable = files.resolve_command("parse_commands");
        let stdout = TypeScriptBridge::execute_with_node(
            &files.git_root,
            format!("{executable} --paths {paths} --root {}", files.git_root).as_str(),
        );
        let result: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&stdout);
        match result {
            Ok(commands) => RepoKitCommand::from_input(&files.git_root, node, commands),
            Err(_) => {
                RepoKitCommand::on_parsing_error(&files.git_root, Value::Null);
                PostProcessor::get().flush();
                exit(0);
            }
        }
    }

    fn execute_with_node(root: &str, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(Path::new(root))
        })
    }
}
