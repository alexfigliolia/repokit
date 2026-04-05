use std::collections::HashMap;

use crate::{
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct Onboarder {
    pub definition: InternalExecutableDefinition,
}

impl Onboarder {
    pub fn new() -> Onboarder {
        Onboarder {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "onboard",
                description: "Onboarding instructions for first time users",
                args: [],
            }),
        }
    }
}

impl InternalExecutable for Onboarder {
    fn run(&self, _args: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info(
            format!(
                "Welcome to {}",
                Logger::with_theme(|theme| theme.highlight("Repokit"))
            )
            .as_str(),
        );
        Logger::info(
            "Repokit is a tool designed to self-document and publish developer facing workflows in a single CLI",
        );
        Logger::info(
            format!("As you develop new features in your codebase, you can publish commands, API's, and tools to the {} CLI by running", Logger::with_theme(|theme|theme.highlight("Repokit"))).as_str()
        );
        Logger::log_file_path("repokit register ./path/to/your-feature");
        Logger::info(
            "This command creates a tooling definition for your feature designed to live along side the feature's implementation",
        );
        Logger::info(
            format!(
                "The {} CLI will automatically detect these files and add them to its toolchain - making them available to your entire team",
                Logger::with_theme(|theme| theme.highlight("Repokit"))
            )
            .as_str(),
        );
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
