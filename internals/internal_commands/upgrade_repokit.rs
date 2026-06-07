use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use crate::{
    context::{file_system::PACKAGE_NAME, node_scope::NodeScope},
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
};

pub static REPOKIT_PACKAGE: LazyLock<String> =
    LazyLock::new(|| format!("{}@latest", *PACKAGE_NAME));

pub struct UpgradeRepoKit {
    pub definition: InternalExecutableDefinition,
}

impl UpgradeRepoKit {
    pub fn new() -> UpgradeRepoKit {
        UpgradeRepoKit {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "upgrade",
                description: "Upgrades your installation of repokit to the latest stable version",
                args: [],
            }),
        }
    }

    pub fn install_latest_repokit(node_scope: &NodeScope, working_directory: &PathBuf) -> bool {
        let success =
            node_scope.install_package(&REPOKIT_PACKAGE, |cmd| cmd.current_dir(working_directory));
        if success {
            return true;
        }
        Logger::info(
            "Something went wrong during the installation. Here's the command I attempted to run",
        );
        Logger::log_file_path(
            format!("{} {}", node_scope.install_command, *REPOKIT_PACKAGE).as_str(),
        );
        false
    }
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Upgrading installation");
        RepoKitRuntime::with_runtime(|runtime| {
            if UpgradeRepoKit::install_latest_repokit(
                &runtime.node,
                &runtime.installation.install_path,
            ) {
                Logger::info("Upgrade Complete!");
            }
        });
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
