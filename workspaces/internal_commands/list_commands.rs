use std::collections::HashMap;

use crate::{
    devkit::interfaces::{DevKitCommand, DevKitConfig},
    executables::{
        intenal_executable::InternalExecutable,
        internal_executable_definition::InternalExecutableDefinition,
    },
    internal_commands::help::Help,
    logger::logger::Logger,
    validations::command_validations::CommandValidations,
};

pub struct ListCommands {
    pub root: String,
    pub configuration: DevKitConfig,
    pub definition: InternalExecutableDefinition,
}

static SCOPES: [&str; 4] = ["internal", "registered", "root", "<owner>"];

impl ListCommands {
    pub fn new(root: String, configuration: DevKitConfig) -> ListCommands {
        ListCommands {
            root,
            configuration,
            definition: InternalExecutableDefinition {
                name: "list-commands",
                description: "List commands based on their scope of definition",
                args: HashMap::from([(
                    "<scope>",
                    format!(
                        "The scope of the commands you wish to list. Specify one of {}",
                        Logger::blue(SCOPES.join(" | ").as_str())
                    )
                    .leak() as &'static str,
                )]),
            },
        }
    }

    fn collect_registered_commands(&self) -> HashMap<String, DevKitCommand> {
        let validators = CommandValidations::new(self.root.clone(), self.configuration.clone());
        validators.collect_and_validate_externals()
    }

    fn exit_on_invalid_scope(&self) {
        Logger::exit_with_info(
            format!(
                "Please specify a scope to list the commands of. Select one of {}",
                Logger::blue_bright(SCOPES.join(" | ").as_str())
            )
            .as_str(),
        );
    }
}

impl InternalExecutable for ListCommands {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>) {
        if args.is_empty() {
            return self.exit_on_invalid_scope();
        }
        let query = args[0].as_str();
        let scope = &query.to_lowercase();
        if scope == SCOPES[0] {
            return Help::log_internal_commands(internals);
        }
        if scope == SCOPES[2] {
            return Help::log_root_commands(&self.configuration.commands);
        }
        let registered_commands = self.collect_registered_commands();
        if scope == SCOPES[1] {
            return Help::log_external_commands(&registered_commands);
        }
        let full_query = args.join(" ");
        let full_scope = &full_query.to_lowercase();
        Logger::info("Searching registered commands");
        let matches: HashMap<String, DevKitCommand> = registered_commands
            .iter()
            .filter_map(|(name, x)| {
                if x.owner.to_lowercase().contains(full_scope) {
                    return Some((name.clone(), x.clone()));
                }
                None
            })
            .collect();
        if matches.is_empty() {
            Logger::exit_with_info(
                format!(
                    "I could not find any commands matching {}",
                    Logger::blue_bright(&full_query)
                )
                .as_str(),
            );
        }
        Help::log_external_commands(&matches);
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
