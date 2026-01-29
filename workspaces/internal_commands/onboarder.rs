use std::collections::HashMap;

use crate::{
    devkit::interfaces::DevKitConfig,
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    logger::logger::Logger,
};

pub struct Onboarder {
    pub root: String,
    pub configuration: DevKitConfig,
    pub definition: InternalExecutableDefinition,
}

impl Onboarder {
    pub fn new(root: String, configuration: DevKitConfig) -> Onboarder {
        Onboarder {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "onboard",
                description: "Onboarding instructions for first time users",
                args: HashMap::from([]),
            },
        }
    }
}

impl InternalExecutable for Onboarder {
    fn run(&self, _: Vec<String>) {
        Logger::info(format!("Welcome to {}", Logger::blue_bright("Devkit")).as_str());
        Logger::info(
            "Devkit is a tool designed to self-document and publish developer facing workflows in a single CLI",
        );
        Logger::info(
            format!("As you develop new features in your codebase, you can publish commands, API's, and tools to the {} CLI by running", Logger::blue_bright("Devkit")).as_str()
        );
        Logger::log_file_path("devkit register-command ./path/to/your-feature");
        Logger::info(
            "This command creates a tooling definition for your feature in a file collocated to your code",
        );
        Logger::info(
            format!(
                "The {} CLI will automatically detect these files and add them to its toolchain",
                Logger::blue_bright("Devkit")
            )
            .as_str(),
        );
        Logger::info(
            format!("As your codebase grows, your {} CLI will continue to track all of the published workflows created by your team", Logger::blue_bright("Devkit")).as_str()
        );
        Logger::space_around("It's your living source of knowledge and documentation");
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
