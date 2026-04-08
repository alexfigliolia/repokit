use normalize_path::NormalizePath;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::{internal_filesystem::file_builder::FileBuilder, logger::logger::Logger};

#[derive(Clone)]
pub struct FileSystem {
    pub git_root: String,
    pub git_root_path: PathBuf,
    pub package_directory: PathBuf,
    pub commands_directory: PathBuf,
    pub templates_directory: PathBuf,
    pub install_script_path: PathBuf,
    pub install_script_location: String,
}

impl FileSystem {
    pub fn new(git_root: &str) -> FileSystem {
        let git_root_path = Path::new(&git_root).normalize();
        let install_script_location = "installation/install.sh".to_string();
        let package_directory = FileSystem::join_with(&git_root_path, "node_modules/@repokit/core");
        FileSystem {
            git_root_path,
            git_root: git_root.to_owned(),
            install_script_path: FileSystem::join_with(
                &package_directory,
                &install_script_location,
            ),
            commands_directory: FileSystem::join_with(&package_directory, "dist/commands"),
            templates_directory: FileSystem::join_with(&package_directory, "externals/templates"),
            package_directory,
            install_script_location,
        }
    }

    pub fn join_with(root: &PathBuf, segment: &str) -> PathBuf {
        root.join(segment).normalize()
    }

    pub fn resolve_command(&self, command_name: &str) -> String {
        self.path_buf_to_str(FileSystem::join_with(
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

    fn path_buf_to_str(&self, buffer: PathBuf) -> String {
        buffer
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }
}
