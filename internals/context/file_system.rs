use normalize_path::NormalizePath;
use regex::Regex;

use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::{internal_filesystem::file_builder::FileBuilder, logger::logger::Logger};

#[derive(Clone)]
pub struct FileSystem {
    pub install_path: PathBuf,
    pub package_directory: PathBuf,
    pub workspace_directory: PathBuf,
    pub commands_directory: PathBuf,
    pub templates_directory: PathBuf,
}

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

impl FileSystem {
    pub fn new(repokit_installation: &PathBuf) -> FileSystem {
        let install_path = repokit_installation.to_owned();
        let package_directory = FileSystem::join_with(&install_path, &INSTALLED_PACKAGE_PATH);
        FileSystem {
            commands_directory: FileSystem::join_with(&package_directory, TYPESCRIPT_COMMANDS),
            templates_directory: FileSystem::join_with(&package_directory, TYPESCRIPT_TEMPLATES),
            workspace_directory: FileSystem::join_with(&install_path, &INSTALLED_WORKSPACE_PATH),
            install_path,
            package_directory,
        }
    }

    pub fn join_with(root: &Path, segment: &str) -> PathBuf {
        root.join(segment).normalize()
    }

    pub fn resolve_command(&self, command_name: &str) -> String {
        FileSystem::path_buf_to_str(&FileSystem::join_with(
            &self.commands_directory,
            format!("{command_name}.mjs").as_str(),
        ))
    }

    pub fn resolve_template(&self, file_name: &str) -> File {
        FileBuilder::open(
            FileSystem::join_with(&self.templates_directory, file_name),
            |_| {
                Logger::error(format!("Unable to locate internal {file_name}").as_str());
                Logger::error("Please file a bug here");
                Logger::log_issue_link();
            },
        )
    }

    pub fn path_buf_to_str(path: &Path) -> String {
        path.to_string_lossy().to_string()
    }
}
