use std::collections::HashMap;

use terminal_spinners::{BOUNCING_BALL, SpinnerBuilder};

use crate::{
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

    pub fn install_at_latest(&self) {
        Logger::info("Upgrading installation");
        let handle = SpinnerBuilder::new()
            .spinner(&BOUNCING_BALL)
            .text(" Installing")
            .start();
        RepoKitRuntime::with_runtime(|runtime| {
            Executor::exec(
                format!("{} @repokit/core@latest", runtime.node.install_command).as_str(),
                |cmd| cmd.current_dir(&runtime.git.root),
            )
        });
        handle.done();
        Logger::info("Upgrade Complete!");
    }
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        self.install_at_latest();
        if let Some(new_version) = RepoKitRuntime::with_runtime(|runtime| {
            runtime
                .caches
                .version_cache
                .refresh_installed_version(&runtime.files)
        }) {
            Logger::info(
                format!(
                    "The currently installed version is {}",
                    Logger::with_theme(|theme| theme.highlight(&new_version))
                )
                .as_str(),
            );
        }
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
