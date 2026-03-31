use normalize_path::NormalizePath;

use regex::Regex;
use shellexpand::tilde;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Lines},
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

    pub fn read_theme_preference(&self) -> String {
        let default = Logger::with_registry(|registry| registry.default_theme.clone());
        let theme = self.read_dot_file(&mut |mut lines, _| {
            if let Some(theme_preference) = lines.nth(1)
                && let Ok(preference) = theme_preference
            {
                return preference;
            }
            default.to_string()
        });
        theme.unwrap_or(default)
    }

    pub fn store_theme_preference(&self, theme: &str) {
        self.read_dot_file(&mut |lines, path| {
            let mut content: Vec<String> = lines.map(|line| line.unwrap()).collect();
            let theme_text = theme.to_string();
            if content.len() >= 2 {
                content[1] = theme_text;
            } else {
                content.push(theme_text);
            }
            fs::write(path, content.join("\n"))
        });
    }

    pub fn read_dot_file<R>(
        &self,
        func: &mut impl FnMut(Lines<BufReader<File>>, PathBuf) -> R,
    ) -> Option<R> {
        let file_path = self.create_dot_file_if_not_exists();
        match file_path {
            Some(path) => {
                if let Ok(file) = File::open(&path) {
                    let lines = BufReader::new(file).lines();
                    return Some(func(lines, path));
                }
                None
            }
            None => None,
        }
    }

    pub fn home() -> Option<PathBuf> {
        let expanded_path_str = tilde("~/");
        let path = Path::new(expanded_path_str.as_ref()).normalize();
        if path.is_absolute() && path.exists() {
            return Some(path);
        }
        None
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

    fn create_dot_file_if_not_exists(&self) -> Option<PathBuf> {
        match InternalFileSystem::home() {
            Some(home) => {
                let dot_file_path = home.join(".repokit");
                if !&dot_file_path.exists() {
                    let found_version = self.current_version();
                    found_version.as_ref()?;
                    let version_string = found_version.unwrap();
                    let result = fs::write(&dot_file_path, format!("{version_string}\n"));
                    if result.is_err() {
                        return None;
                    }
                }
                Some(dot_file_path)
            }
            None => {
                Logger::error(
                    "I encountered an issue when attempting to create a cache file on your machine",
                );
                Logger::error(
                    format!(
                        "Please create a file called {} in your home directory",
                        Logger::with_theme(|theme| theme.highlight(".repokit"))
                    )
                    .as_str(),
                );
                Logger::error(
                    "This file will be used to store settings and indicators that optimize how repokit runs in your repository",
                );
                None
            }
        }
    }

    pub fn current_version(&self) -> Option<String> {
        let package_path = Path::new(&self.root)
            .join(InternalFileSystem::package_directory(self))
            .normalize();
        let package_json_path = package_path.join("package.json");
        let file = File::open(package_json_path);
        if file.is_err() {
            return None;
        }
        let lines = BufReader::new(file.unwrap()).lines();
        let version_matcher = Regex::new(r#""([^"]*)""#).unwrap();
        for line in lines.flatten() {
            if line.contains("\"version\": ") {
                let captures: Vec<String> = version_matcher
                    .captures_iter(&line)
                    .filter_map(|item| {
                        item.get(1)
                            .map(|match_text| match_text.as_str().to_string())
                    })
                    .collect();
                let version = captures.get(1);
                if version.is_some() {
                    return Some(version.unwrap().to_string());
                } else {
                    return None;
                }
            }
        }
        None
    }
}
