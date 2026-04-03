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

    pub fn static_execute(&self) {
        Logger::info("Upgrading installation");
        let handle = SpinnerBuilder::new()
            .spinner(&BOUNCING_BALL)
            .text(" Installing")
            .start();
        let command_prefix = self.scope.node.get_install_command();
        Executor::exec(
            format!("{} @repokit/core@latest", command_prefix).as_str(),
            |cmd| cmd.current_dir(&self.scope.git.root),
        );
        handle.done();
        Logger::info("Upgrade Complete!");
    }
}

impl InternalExecutable for UpgradeRepoKit {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        self.static_execute();
        if let Some(new_version) = self.scope.versions.refresh_installed_version() {
            Logger::info(
                format!(
                    "The currently installed version is {}",
                    Logger::with_theme(|theme| theme.highlight(&new_version))
                )
                .as_str(),
            );
        }
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
