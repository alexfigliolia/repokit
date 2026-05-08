use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use crate::{
    context::file_system::{FileSystem, VERSION_REGEX},
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    executor::executor::Executor,
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
};

pub struct ListVersion {
    pub definition: InternalExecutableDefinition,
}

impl ListVersion {
    pub fn new() -> ListVersion {
        ListVersion {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "version",
                description: "Lists the version of repokit running in this repository",
                args: [],
            }),
        }
    }

    fn log_version(&self, version: &str) {
        Logger::info(format!("{}", Logger::with_theme(|theme| theme.highlight(version))).as_str());
    }

    pub fn get_installed_version(files: &FileSystem) -> Option<String> {
        let package_path = FileSystem::join_with(&files.package_directory, "package.json");
        if !package_path.exists() || !package_path.is_file() {
            return None;
        }
        let file = File::open(package_path);
        if file.is_err() {
            return None;
        }
        let lines = BufReader::new(file.unwrap()).lines();
        let version_matcher = Regex::new(r#""([^"]*)""#).unwrap();
        for line in lines.map_while(Result::ok) {
            if line.contains("\"version\": ") {
                let captures: Vec<String> = version_matcher
                    .captures_iter(&line)
                    .filter_map(|item| {
                        item.get(1)
                            .map(|match_text| match_text.as_str().to_string())
                    })
                    .collect();
                if let Some(version) = captures.get(1)
                    && VERSION_REGEX.is_match(version)
                {
                    return Some(version.to_string());
                }
                return None;
            }
        }
        None
    }
}

impl InternalExecutable for ListVersion {
    fn run(&self, _: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        Logger::info("Fetching the installed version of repokit");
        RepoKitRuntime::with_runtime(|runtime| {
            if let Some(version) = ListVersion::get_installed_version(&runtime.files) {
                return self.log_version(&version);
            }
            Executor::with_stdio("npm list @repokit/native-test", |cmd| cmd);
        });
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
