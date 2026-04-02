use std::collections::HashMap;

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

pub struct ListVersion {
    pub scope: RepoKitScope,
    pub definition: InternalExecutableDefinition,
}

impl ListVersion {
    pub fn new(scope: &RepoKitScope) -> ListVersion {
        ListVersion {
            scope: scope.clone(),
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "version",
                description: "Lists the version of repokit running in this repository",
                args: [],
            }),
        }
    }

    fn log_version(&self, version: &str) {
        Logger::info(format!("{}", Logger::with_theme(|theme| theme.highlight(version))).as_str());
    }
}

impl InternalExecutable for ListVersion {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Fetching the installed version of repokit");
        if let Some(local_version) =
            InternalFileSystem::new(&self.scope.git.root).installed_repokit_version()
        {
            return self.log_version(&local_version);
        }
        Logger::info("Falling back to the runtime version");
        if let Some(runtime_version) = InternalFileSystem::runtime_repokit_version() {
            return self.log_version(&runtime_version);
        }
        Executor::with_stdio("npm list @repokit/core", |cmd| cmd);
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
