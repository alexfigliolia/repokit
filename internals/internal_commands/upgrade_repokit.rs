use normalize_path::NormalizePath;
use std::{collections::HashMap, path::Path, process::exit};

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput, RepoKitScope,
        },
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct UpgradeRepoKit {
    pub scope: RepoKitScope,
    pub definition: InternalExecutableDefinition,
}

impl UpgradeRepoKit {
    pub fn new(scope: &RepoKitScope) -> UpgradeRepoKit {
        UpgradeRepoKit {
            scope: scope.clone(),
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "upgrade",
                description: "Upgrades your installation of repokit to the latest stable version",
                args: [],
            }),
        }
    }

    fn get_package_manager(&self) -> &str {
        let manager_map = HashMap::from([
            ("npm", ("package-lock.json", "npm i -D")),
            ("yarn", ("yarn.lock", "yarn add -D")),
            ("pnpm", ("pnpm-lock.yaml", "pnpm i -D")),
            ("bun", ("bun.lockb", "bun add -d")),
        ]);
        for (manager, (lock_file, command_prefix)) in manager_map {
            let path = Path::new(&self.scope.root).join(lock_file).normalize();
            if path.exists() && path.is_file() {
                Logger::info(
                    format!(
                        "Detected {} installation",
                        Logger::with_theme(|theme| theme.highlight(manager))
                    )
                    .as_str(),
                );
                return command_prefix;
            }
        }
        Logger::info("A node package manager was not detected");
        Logger::info(
            "To upgrade repokit install the latest version using the package manager of your choosing",
        );
        exit(0);
    }
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Upgrading installation");
        let command_prefix = self.get_package_manager();
        Executor::exec(
            format!("{} @repokit/core@latest", command_prefix).as_str(),
            |cmd| cmd.current_dir(&self.scope.root),
        );
        Logger::info("Upgrade complete!");
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
