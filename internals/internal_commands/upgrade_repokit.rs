use std::collections::HashMap;

use crate::{
    context::file_system::PACKAGE_NAME,
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    executor::executor::Executor,
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
};

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
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Upgrading installation");
        RepoKitRuntime::with_runtime(|runtime| {
            Executor::with_stdio(
                format!(
                    "{} {}@latest",
                    runtime.node.install_command,
                    PACKAGE_NAME.as_str()
                )
                .as_str(),
                |cmd| cmd.current_dir(&runtime.installation.install_path),
            )
        });
        Logger::info("Upgrade Complete!");
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
