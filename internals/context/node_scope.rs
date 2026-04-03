use std::{collections::HashMap, path::Path};

use normalize_path::NormalizePath;

use crate::{context::repokit_version_scope::VERSION_REGEX, executor::executor::Executor};

#[derive(Clone)]
pub struct NodeScope {
    pub root: String,
}

impl NodeScope {
    pub fn new(git_root: &str) -> NodeScope {
        NodeScope {
            root: git_root.to_owned(),
        }
    }

    pub fn get_install_command(&self) -> &str {
        let npm_install = "npm i -D";
        let package_manager = self.get_package_manager();
        let manager_map = HashMap::from([
            ("npm", npm_install),
            ("yarn", "yarn add -D"),
            ("pnpm", "pnpm i -D"),
            ("bun", "bun add -D"),
        ]);
        manager_map.get(package_manager).unwrap_or(&npm_install)
    }

    pub fn get_node_executor(&self) -> &str {
        let npx = "npx";
        let package_manager = self.get_package_manager();
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

    fn get_package_manager(&self) -> &str {
        let manager_map = HashMap::from([
            ("npm", "package-lock.json"),
            ("yarn", "yarn.lock"),
            ("pnpm", "pnpm-lock.yaml"),
            ("bun", "bun.lockb"),
        ]);
        for (manager, lock_file) in manager_map {
            let path = Path::new(&self.root).join(lock_file).normalize();
            if path.exists() && path.is_file() {
                return manager;
            }
        }
        "npm"
    }
}
