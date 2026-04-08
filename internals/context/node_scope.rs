use std::{collections::HashMap, path::Path};

use normalize_path::NormalizePath;

use crate::{
    caches::version_cache::VERSION_REGEX, executor::executor::Executor, logger::logger::Logger,
};

#[derive(Clone)]
pub struct NodeScope {
    pub install_command: String,
    pub command_executor: String,
    pub package_manager: String,
    pub resolved_typescript_version: Option<u32>,
}

impl NodeScope {
    pub fn new(git_root: &str) -> NodeScope {
        let package_manager = NodeScope::get_package_manager(git_root).to_string();
        NodeScope {
            resolved_typescript_version: None,
            command_executor: NodeScope::get_node_executor(&package_manager).to_string(),
            install_command: NodeScope::get_install_command(&package_manager).to_string(),
            package_manager,
        }
    }

    pub fn type_check_file(&mut self, file_path: &str) {
        let command = self.get_typecheck_command(file_path);
        Executor::with_stdio(command, |cmd| cmd);
    }

    pub fn get_typecheck_command(&mut self, file_path: &str) -> String {
        let typescript_version = self.get_typescript_version();
        let ignore_config = if typescript_version >= 6 {
            " --ignoreConfig".to_string()
        } else {
            "".to_string()
        };
        let tsc_command = format!(
            "{} tsc {} --noEmit{}",
            self.command_executor, file_path, ignore_config
        );
        tsc_command
    }

    pub fn prompt_to_fix_errors(config_path: &str) {
        Logger::info(
            "Please fix the above type-errors and rerun your command"
                .to_string()
                .as_str(),
        );
        Logger::log_file_path(config_path);
    }

    fn get_typescript_version(&mut self) -> u32 {
        if let Some(resolved_version) = self.resolved_typescript_version {
            return resolved_version;
        }
        let stdout = Executor::exec(format!("{} tsc --version", self.command_executor), |cmd| {
            cmd
        });
        let lines: Vec<&str> = stdout
            .split("\n")
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return None;
                }
                Some(trimmed)
            })
            .collect();
        let fallback_version = "5.0.0";
        let version = lines.last().unwrap_or(&fallback_version);
        let captures: Vec<String> = VERSION_REGEX
            .captures_iter(version)
            .filter_map(|item| {
                item.get(0)
                    .map(|match_text| match_text.as_str().to_string())
            })
            .collect();
        let fallback_version_str = fallback_version.to_string();
        let semver = captures.first().unwrap_or(&fallback_version_str);
        let major = semver
            .chars()
            .next()
            .unwrap_or('5')
            .to_digit(10)
            .unwrap_or(5);
        self.resolved_typescript_version = Some(major);
        major
    }

    fn get_install_command(package_manager: &str) -> &str {
        let npm_install = "npm i -D";
        let manager_map = HashMap::from([
            ("npm", npm_install),
            ("yarn", "yarn add -D"),
            ("pnpm", "pnpm i -D"),
            ("bun", "bun add -D"),
        ]);
        manager_map.get(package_manager).unwrap_or(&npm_install)
    }

    fn get_node_executor(package_manager: &str) -> &str {
        let npx = "npx";
        let manager_map = HashMap::from([
            ("npm", "npx"),
            ("yarn", "yarn run -T"),
            ("pnpm", "pnpm run"),
            ("bun", "bunx"),
        ]);
        manager_map.get(package_manager).unwrap_or(&npx)
    }

    fn get_package_manager(root: &str) -> &str {
        let manager_map = HashMap::from([
            ("npm", ["package-lock.json"].to_vec()),
            ("yarn", ["yarn.lock"].to_vec()),
            ("pnpm", ["pnpm-lock.yaml"].to_vec()),
            ("bun", ["bun.lockb", "bun.lock"].to_vec()),
        ]);
        for (manager, lock_files) in manager_map {
            for lock_file in lock_files {
                let path = Path::new(root).join(lock_file).normalize();
                if path.exists() && path.is_file() {
                    return manager;
                }
            }
        }
        "npm"
    }
}
