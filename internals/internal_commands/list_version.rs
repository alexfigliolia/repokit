use std::collections::HashMap;

use crate::{
    context::repokit_version_scope::VERSION_REGEX,
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    executor::executor::Executor,
    internal_commands::help::Help,
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
};

pub struct ListVersion {
    pub definition: InternalExecutableDefinition,
}

impl ListVersion {
    pub fn new() -> ListVersion {
        ListVersion {
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
        if RepoKitRuntime::with_runtime(|runtime| {
            if VERSION_REGEX.is_match(&runtime.versions.installed_version) {
                self.log_version(&runtime.versions.installed_version);
                return true;
            }
            if VERSION_REGEX.is_match(&runtime.versions.runtime_version) {
                Logger::info("Falling back to the runtime version");
                self.log_version(&runtime.versions.runtime_version);
                return true;
            }
            false
        }) {
            return;
        }
        Executor::with_stdio("npm list @repokit/core", |cmd| cmd);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
