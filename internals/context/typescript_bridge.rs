use core::panic;
use std::{
    path::{Path, PathBuf},
    sync::MutexGuard,
};

use serde_json::{Value, from_str};

use crate::{
    context::{file_system::FileSystem, node_scope::NodeScope},
    executor::executor::Executor,
    repokit::{
        repokit_command::RepoKitCommand, repokit_config::RepoKitConfig,
        repokit_runtime::RepoKitRuntime,
    },
};

pub struct TypeScriptBridge;

impl TypeScriptBridge {
    pub fn parse_configuration(files: &FileSystem, node: &mut NodeScope) -> RepoKitConfig {
        let executable = files.resolve_command("parse_configuration");
        let stdout = TypeScriptBridge::execute_with_node(
            &files.git_root_path,
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
                panic!();
            }
        }
    }

    pub fn parse_commands(path_list: &MutexGuard<Vec<String>>) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let stdout = RepoKitRuntime::with_runtime(|runtime| {
            let executable = runtime.files.resolve_command("parse_commands");
            TypeScriptBridge::execute_with_node(
                &runtime.files.git_root_path,
                format!(
                    "{executable} --paths {paths} --root {}",
                    runtime.files.git_root
                )
                .as_str(),
            )
        });
        let result: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&stdout);
        match result {
            Ok(commands) => RepoKitCommand::from_input(commands),
            Err(_) => {
                RepoKitCommand::on_parsing_error(Value::Null);
                panic!();
            }
        }
    }

    fn execute_with_node(root: &PathBuf, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(Path::new(root))
        })
    }
}
