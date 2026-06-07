use core::panic;
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, MutexGuard},
};

use regex::Regex;
use serde_json::{Value, from_str};

use crate::{
    context::{
        node_scope::NodeScope, typescript_library_installation::TypeScriptLibraryInstallation,
    },
    executor::executor::Executor,
    logger::logger::Logger,
    repokit::{
        repokit_command::RepoKitCommand, repokit_config::RepoKitConfig,
        repokit_runtime::RepoKitRuntime,
    },
    typescript_library::typescript_commands::TypeScriptCommand,
};

static BRIDGE_PARSING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"=============== REPOKIT PARSE FLAG ===============(.*)=============== REPOKIT PARSE FLAG ==============="#).unwrap()
});

pub struct TypeScriptBridge;

impl TypeScriptBridge {
    pub fn parse_configuration(
        library: &TypeScriptLibraryInstallation,
        node: &mut NodeScope,
    ) -> RepoKitConfig {
        let executable = library.resolve_command(TypeScriptCommand::ParseConfiguration);
        let stdout = TypeScriptBridge::execute_with_node(
            &library.install_path,
            format!(
                "{executable} --root {}",
                &library.install_path.to_string_lossy()
            )
            .as_str(),
        );
        if stdout.is_empty() {
            RepoKitConfig::create(library);
        }
        if let Some(parsed_config) = TypeScriptBridge::unwrap_stdout(&stdout) {
            let parse_result: Result<Value, serde_json::Error> = from_str(parsed_config);
            if let Ok(config) = parse_result {
                return RepoKitConfig::from_input(&library.config_path, node, config);
            }
        }
        Logger::parse_error("configuration", &stdout);
        panic!();
    }

    pub fn parse_commands(path_list: &MutexGuard<Vec<String>>) -> Vec<RepoKitCommand> {
        let paths = path_list.join(",");
        let stdout = RepoKitRuntime::with_runtime(|runtime| {
            let executable = runtime
                .typescript_library
                .resolve_command(TypeScriptCommand::ParseCommands);
            TypeScriptBridge::execute_with_node(
                &runtime.typescript_library.install_path,
                format!(
                    "{executable} --paths {paths} --root {}",
                    runtime.typescript_library.install_path.to_string_lossy()
                )
                .as_str(),
            )
        });
        if let Some(parsed_commands) = TypeScriptBridge::unwrap_stdout(&stdout) {
            let parse_result: Result<Vec<Value>, serde_json::Error> =
                serde_json::from_str(parsed_commands);
            if let Ok(commands) = parse_result {
                return RepoKitCommand::from_input(commands);
            }
        }
        Logger::parse_error("commands", &stdout);
        panic!();
    }

    fn execute_with_node(root: &PathBuf, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(Path::new(root))
        })
    }

    fn unwrap_stdout(stdout: &str) -> Option<&str> {
        if let Some(capture) = BRIDGE_PARSING_REGEX.captures(stdout)
            && let Some(result) = capture.get(1)
        {
            return Some(result.as_str());
        }
        None
    }
}
