use normalize_path::NormalizePath;
use regex::Regex;
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{
    argv::argv::{Argv, ArgvOption, ArgvType},
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
    repokit::{repokit_runtime::RepoKitRuntime, repokit_template::RepoKitTemplate},
    typescript_library::typescript_templates::TypeScriptTemplate,
};

static NULL_ARGS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)\n^(\s+"args": null)$"#).unwrap());

pub struct RegisterCommand {
    pub definition: InternalExecutableDefinition,
}

impl RegisterCommand {
    pub fn new() -> RegisterCommand {
        RegisterCommand {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "register",
                description: "Creates new Repokit commands",
                args: [
                    (
                        "<path>",
                        "A relative path to your preferred command location",
                    ),
                    (
                        "(--template | -t)",
                        "An optional template name configured in your RepoKit configuration",
                    ),
                ],
            }),
        }
    }

    fn parse_command_args(&self, args: Vec<String>) -> (PathBuf, String) {
        if args.is_empty() {
            RegisterCommand::exit_on_missing_path();
        }
        let args = Argv::new(
            [ArgvOption {
                value_type: ArgvType::String,
                short: Some("t"),
                name: "template",
                multiple: None,
            }],
            Some(args),
        );
        let template = args.get_first("template");
        let path_arg = args
            .positionals
            .first()
            .unwrap_or(&"".to_owned())
            .to_owned();
        if path_arg.is_empty() {
            RegisterCommand::exit_on_missing_path();
        }
        let mut path = Path::new(&path_arg).to_path_buf().normalize();
        if !path.is_absolute() {
            let working_dir = current_dir().unwrap_or(RepoKitRuntime::with_runtime(|runtime| {
                runtime
                    .typescript_library
                    .install_path
                    .join(&path_arg)
                    .normalize()
            }));
            path = working_dir.join(path).normalize();
        }
        if !path.exists() {
            FileBuilder::create_dir_all(&path, |_| Logger::file_directory_error());
        }
        if !path.is_dir() {
            RegisterCommand::exit_on_missing_path();
        }
        let command_path = &path.join("Commands.ts");
        if command_path.exists() {
            Logger::error(
                format!(
                    "A {} file already exists in this directory",
                    Logger::with_theme(|theme| theme.highlight("Commands.ts"))
                )
                .as_str(),
            );
            Logger::exit_with_info(format!(
                "You can append additional commands to the existing {} instance or export another one",
                Logger::with_theme(|theme| theme.highlight("RepoKitCommand"))
            ).as_str());
        }
        (command_path.clone(), template)
    }

    fn exit_on_missing_path() {
        Logger::exit_with_error(
            "Please specify a path to a directory relative to the root of your repository",
        );
    }

    fn create_from_template(&self, template: &RepoKitTemplate, target_path: &Path) {
        let json = serde_json::to_string_pretty(&template).unwrap();
        let cleaned = NULL_ARGS_REGEX.replace_all(&json, "");
        Logger::info(&Logger::with_theme(|theme| {
            format!(
                "Registering a new command definition based on the {} template",
                theme.highlight(&template.name)
            )
        }));
        if fs::write(
            target_path,
            format!(
                "{}\n\nexport const Commands = new RepoKitCommand({cleaned});",
                "import { RepoKitCommand } from \"@repokit/core\";",
            ),
        )
        .is_ok()
        {
            return self.log_instructions(target_path);
        }
        Logger::exit_with_error(&Logger::with_theme(|theme| {
            format!(
                "I was unable to scaffold a new command definition. This usually has to do with file system permissions in your repository. If you believe this is a bug within {}, please file an issue here",
                theme.highlight("Repokit")
            )
        }));
    }

    fn log_instructions(&self, path: &Path) {
        Logger::info("Please fill out your command file located at:");
        Logger::log_file_path(&path.to_string_lossy());
    }
}

impl InternalExecutable for RegisterCommand {
    fn run(&self, args: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Registering a new command");
        let (command_path, template_name) = self.parse_command_args(args);
        let source_file = RepoKitRuntime::with_runtime(|runtime| {
            if !template_name.is_empty() && !runtime.configuration.templates.is_empty() {
                for template in &runtime.configuration.templates {
                    if template.name == template_name {
                        self.create_from_template(template, &command_path);
                        return None;
                    }
                }
                Logger::exit_with_info(&Logger::with_theme(|theme| {
                    format!(
                        "A template with the name \"{}\" was not found",
                        theme.highlight(&template_name)
                    )
                }));
            }
            Some(
                runtime
                    .typescript_library
                    .resolve_template(TypeScriptTemplate::CommandTemplate),
            )
        });
        if let Some(mut source) = source_file {
            let mut target = FileBuilder::create(&command_path, |_| Logger::file_create_error());
            FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
            Logger::info("Creating command file");
            self.log_instructions(&command_path);
        }
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
