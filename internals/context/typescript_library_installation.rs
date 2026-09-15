use normalize_path::NormalizePath;
use regex::Regex;
use serde::de::DeserializeOwned;

use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{
    executor::executor::Executor,
    internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
    repokit::{repokit_command::RepoKitCommand, repokit_config::RepoKitConfig},
    typescript_library::{
        file_parse_result::FileParseResult, typescript_commands::TypeScriptCommand,
        typescript_templates::TypeScriptTemplate,
    },
};

static BRIDGE_PARSING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"=============== REPOKIT PARSE FLAG ===============(.*)=============== REPOKIT PARSE FLAG ==============="#).unwrap()
});

static NON_EXISTENT_MATCHER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)does not exist"#).unwrap());

#[derive(Clone)]
pub struct TypeScriptLibraryInstallation {
    pub config_path: PathBuf,
    pub install_path: PathBuf,
    pub package_directory: PathBuf,
    pub workspace_directory: PathBuf,
    pub commands_directory: PathBuf,
    pub templates_directory: PathBuf,
}

pub static CONFIG_FILE_NAME: &str = "repokit.ts";
pub static WORKSPACE_NAME: &str = "@repokit";
pub static PACKAGE_NAME: LazyLock<String> = LazyLock::new(|| format!("{WORKSPACE_NAME}/core"));
pub static INSTALLED_WORKSPACE_PATH: LazyLock<String> =
    LazyLock::new(|| format!("node_modules/{WORKSPACE_NAME}"));
pub static INSTALLED_PACKAGE_PATH: LazyLock<String> =
    LazyLock::new(|| format!("node_modules/{}", PACKAGE_NAME.as_str()));
static TYPESCRIPT_COMMANDS: &str = "dist/commands";
static TYPESCRIPT_TEMPLATES: &str = "externals/templates";
pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d+\.\d+.\d+"#).unwrap());

impl TypeScriptLibraryInstallation {
    pub fn new(repokit_installation: &PathBuf) -> Self {
        let install_path = repokit_installation.to_owned();
        let package_directory =
            TypeScriptLibraryInstallation::join_with(&install_path, &INSTALLED_PACKAGE_PATH);
        Self {
            commands_directory: TypeScriptLibraryInstallation::join_with(
                &package_directory,
                TYPESCRIPT_COMMANDS,
            ),
            templates_directory: TypeScriptLibraryInstallation::join_with(
                &package_directory,
                TYPESCRIPT_TEMPLATES,
            ),
            workspace_directory: TypeScriptLibraryInstallation::join_with(
                &install_path,
                &INSTALLED_WORKSPACE_PATH,
            ),
            config_path: TypeScriptLibraryInstallation::join_with(&install_path, CONFIG_FILE_NAME),
            install_path,
            package_directory,
        }
    }

    pub fn join_with(root: &Path, segment: &str) -> PathBuf {
        root.join(segment).normalize()
    }

    pub fn resolve_command(&self, command_name: TypeScriptCommand) -> String {
        TypeScriptLibraryInstallation::path_buf_to_str(&TypeScriptLibraryInstallation::join_with(
            &self.commands_directory,
            format!("{}.mjs", command_name.resolve()).as_str(),
        ))
    }

    pub fn resolve_template(&self, file_name: TypeScriptTemplate) -> File {
        FileBuilder::open(
            TypeScriptLibraryInstallation::join_with(
                &self.templates_directory,
                file_name.resolve(),
            ),
            |_| {
                Logger::error(
                    format!("Unable to locate internal {}", file_name.resolve()).as_str(),
                );
                Logger::error("Please file a bug here");
                Logger::log_issue_link();
            },
        )
    }

    pub fn path_buf_to_str(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }

    pub fn parse_configuration(&self) -> RepoKitConfig {
        let executable = self.resolve_command(TypeScriptCommand::ParseConfiguration);
        let parse_result = self.unwrap_command_result::<RepoKitConfig>(
            &self.execute_with_node(&format!(
                "{executable} --root \"{}\"",
                self.install_path.to_string_lossy()
            )),
            |stdout| {
                Logger::parse_error("configuration", stdout);
            },
        );
        if let Some(error) = &parse_result.error {
            if let Some(message) = &error.message
                && NON_EXISTENT_MATCHER.is_match(message)
            {
                RepoKitConfig::create(self);
            }
            RepoKitConfig::on_parsing_error(&self.config_path);
        }
        if let Some(config) = parse_result.result {
            return RepoKitConfig::validate(config);
        }
        panic!();
    }

    pub fn parse_commands(&self, paths: &[String]) -> Vec<RepoKitCommand> {
        let executable = self.resolve_command(TypeScriptCommand::ParseCommands);
        let parse_result = self.unwrap_command_result::<Vec<RepoKitCommand>>(
            &self.execute_with_node(&format!(
                "{executable} --paths \"{}\" --root \"{}\"",
                paths.join(","),
                self.install_path.to_string_lossy()
            )),
            |stdout| {
                Logger::parse_error("commands", stdout);
            },
        );
        if let Some(error) = &parse_result.error {
            RepoKitCommand::on_parsing_error(&error.path);
        }
        if let Some(result) = parse_result.result {
            return result;
        }
        panic!();
    }

    fn execute_with_node(&self, args: &str) -> String {
        Executor::exec(format!("node {args}"), |cmd| {
            cmd.current_dir(&self.install_path)
        })
    }

    fn unwrap_command_result<T>(
        &self,
        stdout: &str,
        on_error: impl FnOnce(&str),
    ) -> FileParseResult<T>
    where
        T: DeserializeOwned,
    {
        if let Some(capture) = BRIDGE_PARSING_REGEX.captures(stdout)
            && let Some(result) = capture.get(1)
        {
            let parse_result: Result<FileParseResult<T>, serde_json::Error> =
                serde_json::from_str(result.as_str());
            if let Ok(json) = parse_result {
                return json;
            }
        }
        on_error(stdout);
        panic!();
    }
}
