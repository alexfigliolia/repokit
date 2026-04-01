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
    internal_filesystem::internal_filesystem::{InternalFileSystem, VERSION_REGEX},
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
}

impl InternalExecutable for ListVersion {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Fetching the installed version of repokit");
        if let Some(fallback) = InternalFileSystem::new(&self.scope.root).current_version()
            && VERSION_REGEX.is_match(&fallback)
        {
            return Logger::info(
                format!("{}", Logger::with_theme(|theme| theme.highlight(&fallback))).as_str(),
            );
        }
        let version = Executor::exec("head -n 1 ~/.repokit", |cmd| cmd);
        if VERSION_REGEX.is_match(&version) {
            return Logger::info(
                format!("{}", Logger::with_theme(|theme| theme.highlight(&version))).as_str(),
            );
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
