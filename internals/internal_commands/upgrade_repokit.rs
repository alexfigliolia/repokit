use std::collections::HashMap;

use terminal_spinners::{BOUNCING_BALL, SpinnerBuilder};

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput, RepoKitScope,
        },
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    internal_filesystem::internal_filesystem::InternalFileSystem,
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

    pub fn static_execute(root: &str) {
        Logger::info("Upgrading installation");
        let handle = SpinnerBuilder::new()
            .spinner(&BOUNCING_BALL)
            .text(" Installing")
            .start();
        let command_prefix = InternalFileSystem::get_install_command(root);
        Executor::exec(
            format!("{} @repokit/core@latest", command_prefix).as_str(),
            |cmd| cmd.current_dir(root),
        );
        handle.done();
    }
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        let internal_fs = InternalFileSystem::new(&self.scope.git.root);
        let fallback = "unknown";
        let runtime_version = internal_fs
            .installed_repokit_version()
            .unwrap_or(fallback.to_string());
        UpgradeRepoKit::static_execute(&self.scope.git.root);
        let installed_version = internal_fs
            .installed_repokit_version()
            .unwrap_or(fallback.to_string());
        if runtime_version != installed_version {
            Logger::info("Upgrade Complete!");
            Logger::info(
                format!(
                    "The currently installed version is {}",
                    Logger::with_theme(|theme| theme.highlight(&installed_version))
                )
                .as_str(),
            );
        } else {
            Logger::info("The latest version is already installed");
        }
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
