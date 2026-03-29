use normalize_path::NormalizePath;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::{
    executor::executor::Executor, internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger,
};

pub struct InternalFileSystem {
    root: String,
}

impl InternalFileSystem {
    pub fn new(root: &str) -> InternalFileSystem {
        InternalFileSystem {
            root: root.to_string(),
        }
    }

    pub fn absolute(&self, segment: &str) -> PathBuf {
        let path = Path::new(&self.root);
        path.join(segment).normalize()
    }

    pub fn resolve_command(&self, command_name: &str) -> String {
        self.path_buf_to_str(
            self.commands_directory()
                .join(format!("{command_name}.mjs")),
        )
    }

    pub fn resolve_template(&self, file_name: &str) -> File {
        let path = self.path_buf_to_str(self.templates_directory().join(file_name));
        FileBuilder::open(&path, |_| {
            Logger::error(format!("Unable to locate internal {file_name}").as_str());
            Logger::error("Please file a bug here");
            Logger::log_issue_link();
        })
    }

    pub fn find_root() -> String {
        let root = Executor::exec("echo $(git rev-parse --show-toplevel 2>/dev/null)", |cmd| {
            cmd
        });
        if root.is_empty() {
            Logger::exit_with_info(
                format!(
                    "To start using {}, please initialize your git repository by running {}",
                    Logger::with_theme(|theme| theme.highlight("Repokit")),
                    Logger::with_theme(|theme| theme.highlight("git init"))
                )
                .as_str(),
            );
        }
        root
    }

    fn commands_directory(&self) -> PathBuf {
        self.absolute(format!("{}/dist/commands", self.package_directory()).as_str())
    }

    fn templates_directory(&self) -> PathBuf {
        self.absolute(format!("{}/externals/templates", self.package_directory()).as_str())
    }

    fn package_directory(&self) -> String {
        format!("./node_modules/{}", self.package_name())
    }

    fn package_name(&self) -> String {
        "@repokit/core".to_string()
    }

    fn path_buf_to_str(&self, buffer: PathBuf) -> String {
        buffer
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }
}
