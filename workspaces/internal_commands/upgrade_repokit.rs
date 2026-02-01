use normalize_path::NormalizePath;
use std::{collections::HashMap, path::Path, process::exit};

use crate::{
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::interfaces::RepoKitConfig,
};

pub struct UpgradeRepoKit {
    pub root: String,
    pub configuration: RepoKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl UpgradeRepoKit {
    pub fn new(root: String, configuration: RepoKitConfig) -> UpgradeRepoKit {
        UpgradeRepoKit {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "upgrade",
                description: "Upgrades your installation of repokit to the latest stable version",
                args: HashMap::from([]),
            },
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
            let path = Path::new(&self.root).join(lock_file).normalize();
            if path.exists() && path.is_file() {
                Logger::info(
                    format!("Detected {} installation", Logger::blue_bright(manager)).as_str(),
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
            |cmd| cmd.current_dir(&self.root),
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
