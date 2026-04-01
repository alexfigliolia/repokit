use std::{
    collections::HashMap,
    path::Path,
    process::exit,
    sync::{LazyLock, Mutex},
};

use jsonschema::Validator;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, from_value, to_value};

use crate::{
    configuration::recovery::Recovery,
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
    repokit::{
        command_definition::CommandDefinition, repokit_command::RepoKitCommand,
        repokit_construct_validator::RepoKitConstructValidator,
    },
    themes::theme_inputs::RepoKitTheme,
};

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct RootCommand {
    pub name: String,
    pub command: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}

impl RootCommand {
    pub fn from(name: &str, command: &CommandDefinition) -> RootCommand {
        RootCommand {
            name: name.to_string(),
            args: command.args.clone(),
            command: command.command.to_string(),
            description: command.description.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct RepoKitConfig {
    pub project: String,
    pub thirdParty: Vec<RepoKitCommand>,
    pub commands: HashMap<String, CommandDefinition>,
    pub themes: Vec<RepoKitTheme>,
}

static REPOKIT_CONFIG_VALIDATOR: LazyLock<Mutex<Validator>> = LazyLock::new(|| {
    Mutex::new(Validator::new(&to_value(schemars::schema_for!(RepoKitConfig)).unwrap()).unwrap())
});

impl RepoKitConstructValidator<Value, RepoKitConfig> for RepoKitConfig {
    fn from_input(root: &str, input: Value) -> RepoKitConfig {
        let validator = REPOKIT_CONFIG_VALIDATOR.lock().unwrap();
        let repokit_config: Result<RepoKitConfig, serde_json::Error> = from_value(input.clone());
        if !RepoKitConfig::is_valid(&validator, &input) || repokit_config.is_err() {
            RepoKitConfig::on_parsing_error(root, Value::Null);
        }
        repokit_config.expect("assertions succeeded")
    }

    fn on_parsing_error(root: &str, _: Value) -> Option<String> {
        let path_buf = Path::new(&root).join("repokit.ts");
        let path = path_buf.to_str().expect("exists");
        let mut recovery = Recovery::new(root);
        recovery.run(path);
        println!();
        Logger::info("There was an error parsing your configuration");
        recovery.prompt_to_fix_errors(path);
        PostProcessor::get().flush();
        exit(0);
    }
}
