use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::LazyLock,
};

use futures::join;
use normalize_path::NormalizePath;
use regex::Regex;

use crate::{
    context::{
        file_system::FileSystem, initializer::Initializer, internal_caches::InternalCaches,
        repokit_version_resolver::RepoKitVersionResolver,
    },
    executor::executor::Executor,
    logger::logger::Logger,
};

static UNKNOWN: &str = "unknown";

pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d*\.\d*.\d*"#).unwrap());

#[derive(Clone)]
pub struct RepoKitVersionScope {
    pub files: FileSystem,
    pub runtime_version: String,
    pub installed_version: String,
}

impl Initializer<(Option<String>, Option<String>)> for RepoKitVersionScope {
    async fn resolve(&mut self, _: &str) -> (Option<String>, Option<String>) {
        join!(self.runtime_version(), self.installed_repokit_version())
    }
}

impl RepoKitVersionScope {
    pub fn new(files: &FileSystem) -> RepoKitVersionScope {
        let mut instance = RepoKitVersionScope {
            files: files.clone(),
            runtime_version: UNKNOWN.to_string(),
            installed_version: UNKNOWN.to_string(),
        };
        let (runtime_version, installed_version) =
            RepoKitVersionScope::resolve_sync(instance.resolve(&files.root));
        instance.runtime_version = instance.unwap(runtime_version);
        instance.installed_version = instance.unwap(installed_version);
        instance.hop_to_installed_version();
        instance
    }

    pub fn refresh_installed_version(&self) -> Option<String> {
        let installed_version = RepoKitVersionScope::resolve_sync(self.installed_repokit_version());
        if let Some(version) = &installed_version
            && *version != self.installed_version
        {
            return Some(version.to_string());
        }
        None
    }

    fn unwap(&self, version: Option<String>) -> String {
        version.unwrap_or(UNKNOWN.to_string())
    }

    fn hop_to_installed_version(&self) {
        if self.runtime_version != self.installed_version && self.installed_version != UNKNOWN {
            Logger::info(
                format!(
                    "Switching to version {}",
                    Logger::with_theme(|theme| theme.highlight(&self.installed_version))
                )
                .as_str(),
            );
            RepoKitVersionResolver::hop_to_installed_version(&self.files);
        }
    }

    async fn runtime_version(&self) -> Option<String> {
        if let Some(home) = InternalCaches::home() {
            let version = Executor::exec(
                format!(
                    "head -n 1 {}",
                    home.join(".repokit").normalize().to_str().unwrap()
                ),
                |cmd| cmd,
            );
            if VERSION_REGEX.is_match(&version) {
                return Some(version);
            }
        }
        None
    }

    async fn installed_repokit_version(&self) -> Option<String> {
        let package_path = FileSystem::join_with(&self.files.package_directory, "package.json");
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
