use normalize_path::NormalizePath;
use std::{collections::HashMap, path::Path, process::exit};

use crate::{
    devkit::interfaces::DevKitConfig,
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct UpgradeDevKit {
    pub root: String,
    pub configuration: DevKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl UpgradeDevKit {
    pub fn new(root: String, configuration: DevKitConfig) -> UpgradeDevKit {
        UpgradeDevKit {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "upgrade-devkit",
                description: "Upgrades your installation of devkit to the latest stable version",
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
            "To upgrade devkit install the latest version using the package manager of your choosing",
        );
        exit(0);
    }
}

impl InternalExecutable for UpgradeDevKit {
    fn run(&self, _: Vec<String>) {
        Logger::info("Upgrading installation");
        let command_prefix = self.get_package_manager();
        Executor::exec(
            format!("{} @devkit/cli@latest", command_prefix).as_str(),
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
