use core::panic;
use jsonschema::Validator;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, from_value, to_value};
use std::{
    collections::HashMap,
    path::Path,
    sync::LazyLock,
};

use crate::{
    context::{file_system::FileSystem, node_scope::NodeScope},
    internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
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
    #[serde(rename = "thirdParty")]
    pub third_party: Vec<RepoKitCommand>,
    pub commands: HashMap<String, CommandDefinition>,
    pub themes: Vec<RepoKitTheme>,
}

static REPOKIT_CONFIG_VALIDATOR: LazyLock<Validator> = LazyLock::new(|| {
    Validator::new(&to_value(schemars::schema_for!(RepoKitConfig)).unwrap()).unwrap()
});

impl RepoKitConstructValidator for RepoKitConfig {}

impl RepoKitConfig {
    pub fn from_input(root: &Path, node: &mut NodeScope, input: Value) -> RepoKitConfig {
        let repokit_config: Result<RepoKitConfig, serde_json::Error> = from_value(input.clone());
        if !RepoKitConfig::is_valid(&REPOKIT_CONFIG_VALIDATOR, &input) || repokit_config.is_err() {
            RepoKitConfig::on_parsing_error(root, node, Value::Null);
        }
        repokit_config.expect("assertions succeeded")
    }

    pub fn on_parsing_error(root: &Path, node: &mut NodeScope, _: Value) -> Option<String> {
        let path_buf = root.join("repokit.ts");
        node.type_check_file(&path_buf);
        println!();
        Logger::info("There was an error parsing your configuration");
        NodeScope::prompt_to_fix_errors(&path_buf);
        panic!();
    }

    pub fn create(files: &FileSystem) {
        let file_path = &files.install_path.join("repokit.ts");
        if file_path.exists() {
            Logger::info(
                format!(
                    "I found a Repokit configuration but could not resolve the exported {} instance",
                    Logger::with_theme(|theme| theme.highlight("RepokitConfig"))
                )
                .as_str(),
            );
            Logger::exit_with_info(format!(
                "Please double check that your config file exports a {} instance and is free of any sideffects that can cause the runtime to crash",
                Logger::with_theme(|theme| theme.highlight("RepokitConfig"))
            ).as_str());
        }
        Logger::info("Welcome to Repokit! Let's get you setup");
        Logger::info("Creating your configuration file:");
        let mut source = files.resolve_template("configuration_template.txt");
        let mut target = FileBuilder::create(file_path, |_| Logger::file_create_error());
        FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
        Logger::info(
            format!(
                "Please fill out this file with your desired settings. Then run {}",
                Logger::with_theme(|theme| theme.highlight("repokit onboard"))
            )
            .as_str(),
        );
        Logger::log_file_path(&file_path.to_string_lossy());
        panic!();
    }
}
