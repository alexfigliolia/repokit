use core::panic;
use std::{
    path::{Path, PathBuf},
    sync::{LazyLock, MutexGuard},
};

use futures::{executor::block_on, future::join_all};
use regex::Regex;
use serde_json::{Value, from_str};
use tokio::runtime::Builder;

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
                "{executable} --root \"{}\"",
                library.install_path.to_string_lossy()
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
        let (executable, root) = RepoKitRuntime::with_runtime(|runtime| {
            let executable = runtime
                .typescript_library
                .resolve_command(TypeScriptCommand::ParseCommands);
            let install_path = &runtime.typescript_library.install_path;
            (executable, install_path.to_owned())
        });
        let process_distributions = TypeScriptBridge::calculate_process_distribution(path_list);
        let parse_tasks = process_distributions
            .iter()
            .map(|batch| TypeScriptBridge::collect_command_batch(&root, &executable, batch));
        let command_definitions = block_on(join_all(parse_tasks));

        TypeScriptBridge::multi_thread_definition_parsing(command_definitions)
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

    fn calculate_process_distribution(path_list: &MutexGuard<Vec<String>>) -> Vec<Vec<String>> {
        let total_paths = path_list.len();
        let mut result = Vec::<Vec<String>>::new();
        if total_paths < 10000 {
            result.push(path_list.iter().map(|e| e.to_owned()).collect());
            return result;
        }
        let cpus = num_cpus::get_physical().div_ceil(2);
        let distribution_per_process = total_paths.div_ceil(cpus);
        for cpu_idx in 0..cpus {
            let mut paths_for_process = Vec::new();
            for path_idx in 0..distribution_per_process {
                if let Some(path) = path_list.get(path_idx + (cpu_idx * distribution_per_process)) {
                    paths_for_process.push(path.to_owned());
                }
            }
            result.push(paths_for_process);
        }
        result
    }

    async fn collect_command_batch(root: &PathBuf, executable: &str, batch: &[String]) -> String {
        TypeScriptBridge::execute_with_node(
            root,
            format!(
                "{executable} --paths \"{}\" --root \"{}\"",
                batch.join(","),
                root.to_string_lossy()
            )
            .as_str(),
        )
    }

    fn multi_thread_definition_parsing(command_definitions: Vec<String>) -> Vec<RepoKitCommand> {
        let total_batches = command_definitions.len();
        if total_batches == 0 {
            return Vec::new();
        }
        if total_batches == 1 {
            return TypeScriptBridge::json_parse_command_batch(
                command_definitions.first().unwrap(),
            );
        }
        let mut json_tasks = Vec::new();
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        for batch in command_definitions {
            json_tasks.push(
                runtime.spawn(async move { TypeScriptBridge::json_parse_command_batch(&batch) }),
            )
        }
        let results: Vec<RepoKitCommand> = block_on(join_all(json_tasks))
            .iter()
            .flat_map(|f| f.as_ref().unwrap().to_owned())
            .collect();
        results
    }

    fn json_parse_command_batch(batch: &str) -> Vec<RepoKitCommand> {
        if let Some(parsed_commands) = TypeScriptBridge::unwrap_stdout(batch) {
            let parse_result: Result<Vec<Value>, serde_json::Error> =
                serde_json::from_str(parsed_commands);
            if let Ok(commands) = parse_result {
                return RepoKitCommand::from_input(commands);
            }
        }
        Logger::parse_error("commands", batch);
        panic!();
    }
}
