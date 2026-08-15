use std::collections::HashMap;

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    logger::logger::Logger,
};

pub static REPOKIT_VERSION: &str = "5.1.5";

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
}

impl InternalExecutable for ListVersion {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Fetching the installed version of repokit");
        Logger::info(
            format!(
                "{}",
                Logger::with_theme(|theme| theme.highlight(REPOKIT_VERSION))
            )
            .as_str(),
        );
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}