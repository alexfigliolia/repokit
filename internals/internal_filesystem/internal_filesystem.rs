use normalize_path::NormalizePath;

use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{
    executor::executor::Executor, internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
};

pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d*\.\d*.\d*"#).unwrap());

pub struct InternalFileSystem {
    root: String,
}

impl InternalFileSystem {
    pub fn new(root: &str) -> InternalFileSystem {
        InternalFileSystem {
            root: root.to_string(),
        }
    }

    pub fn absolute(&self, segment: &str) -> PathBuf {
        let path = Path::new(&self.root);
        path.join(segment).normalize()
    }

    pub fn resolve_command(&self, command_name: &str) -> String {
        self.path_buf_to_str(
            self.commands_directory()
                .join(format!("{command_name}.mjs")),
        )
    }

    pub fn resolve_template(&self, file_name: &str) -> File {
        let path = self.path_buf_to_str(self.templates_directory().join(file_name));
        FileBuilder::open(&path, |_| {
            Logger::error(format!("Unable to locate internal {file_name}").as_str());
            Logger::error("Please file a bug here");
            Logger::log_issue_link();
        })
    }

    pub fn get_package_manager(root: &str) -> &str {
        let manager_map = HashMap::from([
            ("npm", "package-lock.json"),
            ("yarn", "yarn.lock"),
            ("pnpm", "pnpm-lock.yaml"),
            ("bun", "bun.lockb"),
        ]);
        for (manager, lock_file) in manager_map {
            let path = Path::new(&root).join(lock_file).normalize();
            if path.exists() && path.is_file() {
                return manager;
            }
        }
        "npm"
    }

    pub fn get_install_command(root: &str) -> &str {
        let npm_install = "npm i -D";
        let package_manager = InternalFileSystem::get_package_manager(root);
        let manager_map = HashMap::from([
            ("npm", npm_install),
            ("yarn", "yarn add -D"),
            ("pnpm", "pnpm i -D"),
            ("bun", "bun add -D"),
        ]);
        manager_map.get(package_manager).unwrap_or(&npm_install)
    }

    pub fn get_node_executor(root: &str) -> &str {
        let npx = "npx";
        let package_manager = InternalFileSystem::get_package_manager(root);
        let manager_map = HashMap::from([
            ("npm", "npx"),
            ("yarn", "yarn run -T"),
            ("pnpm", "pnpm run"),
            ("bun", "bunx"),
        ]);
        manager_map.get(package_manager).unwrap_or(&npx)
    }

    pub fn get_typescript_version(node_executor: &str) -> u32 {
        let stdout = Executor::exec(format!("{} tsc --version", node_executor), |cmd| cmd);
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
        semver
            .chars()
            .next()
            .unwrap_or('5')
            .to_digit(10)
            .unwrap_or(5)
    }

    fn commands_directory(&self) -> PathBuf {
        self.absolute(format!("{}/dist/commands", self.package_directory()).as_str())
    }

    fn templates_directory(&self) -> PathBuf {
        self.absolute(format!("{}/externals/templates", self.package_directory()).as_str())
    }

    pub fn package_directory(&self) -> String {
        format!("./node_modules/{}", self.package_name())
    }

    fn package_name(&self) -> String {
        "@repokit/core".to_string()
    }

    fn path_buf_to_str(&self, buffer: PathBuf) -> String {
        buffer
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }
}
