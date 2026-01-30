use std::collections::HashMap;

use crate::{
    devkit::interfaces::DevKitConfig,
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

static SCOPES: [&str; 3] = ["internal", "external", "root"];

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
                        Logger::blue_bright(SCOPES.join(" | ").as_str())
                    )
                    .leak() as &'static str,
                )]),
            },
        }
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
        let scope = args[0].as_str();
        if !SCOPES.contains(&scope) {
            return self.exit_on_invalid_scope();
        }
        if scope == SCOPES[0] {
            return Help::log_internal_commands(internals);
        }
        if scope == SCOPES[1] {
            let validators = CommandValidations::new(self.root.clone(), self.configuration.clone());
            let externals = validators.collect_and_validate_externals();
            return Help::log_external_commands(&externals);
        }
        if scope == SCOPES[2] {
            return Help::log_root_commands(&self.configuration.commands);
        }
        self.exit_on_invalid_scope()
    }

    fn help(&self) {
        Help::log_internal_command(&self.definition);
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
