use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::LazyLock,
};

use futures::{executor::block_on, join};
use regex::Regex;

use crate::{
    caches::{file_cache::FileCache, repokit_version_resolver::RepoKitVersionResolver},
    context::file_system::FileSystem,
    logger::logger::Logger,
};

#[derive(Clone)]
pub struct VersionCache {
    pub runtime_version: String,
    pub installed_version: String,
    pub cache_directory: Option<PathBuf>,
}

static UNKNOWN: &str = "UNKNOWN";

pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d*\.\d*.\d*"#).unwrap());

impl VersionCache {
    pub fn new(cache_directory: &Option<PathBuf>) -> Self {
        VersionCache {
            runtime_version: UNKNOWN.to_string(),
            installed_version: UNKNOWN.to_string(),
            cache_directory: cache_directory.clone(),
        }
    }

    pub fn refresh_installed_version(&self, files: &FileSystem) -> Option<String> {
        let installed_version = block_on(self.installed_repokit_version(files));
        if let Some(version) = &installed_version
            && *version != self.installed_version
        {
            return Some(version.to_string());
        }
        None
    }

    pub async fn initialize(&mut self, files: &FileSystem) {
        let (runtime_result, install_result) = join!(
            self.runtime_version(),
            self.installed_repokit_version(files)
        );
        if let Some(installed_version) = install_result {
            self.installed_version = installed_version;
        }
        if let Some(runtime_version) = runtime_result {
            self.runtime_version = runtime_version;
        }
        if self.installed_version != self.runtime_version
            && VERSION_REGEX.is_match(&self.installed_version)
        {
            self.hop_to_installed_version(files);
        }
    }

    fn hop_to_installed_version(&self, files: &FileSystem) {
        Logger::info(
            format!(
                "Switching to version {}",
                Logger::with_theme(|theme| theme.highlight(&self.installed_version))
            )
            .as_str(),
        );
        RepoKitVersionResolver::hop_to_installed_version(files);
    }

    async fn runtime_version(&self) -> Option<String> {
        if let Some((mut lines, _)) = self.read() {
            let last_known_version = self.unwrap_line(lines.nth(0), "");
            if VERSION_REGEX.is_match(&last_known_version) {
                return Some(last_known_version);
            }
            return None;
        }
        None
    }

    async fn installed_repokit_version(&self, files: &FileSystem) -> Option<String> {
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

impl FileCache for VersionCache {
    fn cache_file(&self) -> &str {
        "../.version"
    }

    fn cache_directory(&self) -> &Option<PathBuf> {
        &self.cache_directory
    }
}
