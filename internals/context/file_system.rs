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
    pub git_root: String,
    pub git_root_path: PathBuf,
    pub package_directory: PathBuf,
    pub commands_directory: PathBuf,
    pub templates_directory: PathBuf,
}

static INSTALLED_PACKAGE_PATH: &str = "node_modules/@repokit/core";
static TYPESCRIPT_COMMANDS: &str = "dist/commands";
static TYPESCRIPT_TEMPLATES: &str = "externals/templates";
pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d*\.\d*.\d*"#).unwrap());

impl FileSystem {
    pub fn new(git_root: &str) -> FileSystem {
        let git_root_path = Path::new(&git_root).normalize();
        let package_directory = FileSystem::join_with(&git_root_path, INSTALLED_PACKAGE_PATH);
        FileSystem {
            git_root_path,
            git_root: git_root.to_owned(),
            commands_directory: FileSystem::join_with(&package_directory, TYPESCRIPT_COMMANDS),
            templates_directory: FileSystem::join_with(&package_directory, TYPESCRIPT_TEMPLATES),
            package_directory,
        }
    }

    pub fn join_with(root: &PathBuf, segment: &str) -> PathBuf {
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

    pub fn path_buf_to_str(path: &PathBuf) -> String {
        path.to_string_lossy().to_string()
    }
}
