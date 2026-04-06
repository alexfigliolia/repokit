use core::panic;
use std::{
    fs::{File, write},
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::LazyLock,
};

use futures::join;
use regex::Regex;

use crate::{
    context::{
        cache_scope::CacheScope, file_system::FileSystem, initializer::Initializer,
        internal_caches::Cache, repokit_version_resolver::RepoKitVersionResolver,
    },
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

static UNKNOWN: &str = "UNKNOWN";

pub static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\d*\.\d*.\d*"#).unwrap());

#[derive(Clone)]
pub struct RepoKitVersionScope {
    pub runtime_version: String,
    pub installed_version: String,
}

impl Initializer<(), (&FileSystem, &CacheScope)> for RepoKitVersionScope {
    async fn resolve(&mut self, resolvers: (&FileSystem, &CacheScope)) {
        let (files, cache) = resolvers;
        let (runtime_result, install_result) = join!(
            self.runtime_version(cache),
            self.installed_repokit_version(files)
        );
        if let Some(installed_version) = install_result {
            self.installed_version = installed_version;
        }
        if let Some(runtime_version) = runtime_result {
            self.runtime_version = runtime_version;
        } else {
            self.runtime_version = self.installed_version.clone();
        }
    }
}

impl RepoKitVersionScope {
    pub fn new(resolvers: (&FileSystem, &CacheScope)) -> RepoKitVersionScope {
        let mut instance = RepoKitVersionScope {
            runtime_version: UNKNOWN.to_string(),
            installed_version: UNKNOWN.to_string(),
        };
        let (files, cache) = resolvers;
        RepoKitVersionScope::resolve_sync(instance.resolve(resolvers));
        if instance.installed_version != instance.runtime_version
            && VERSION_REGEX.is_match(&instance.installed_version)
        {
            instance.hop_to_installed_version(files, cache);
        }
        instance
    }

    pub fn refresh_installed_version(&self, files: &FileSystem) -> Option<String> {
        let installed_version =
            RepoKitVersionScope::resolve_sync(self.installed_repokit_version(files));
        if let Some(version) = &installed_version
            && *version != self.installed_version
        {
            return Some(version.to_string());
        }
        None
    }

    fn record_version_use(&self, cache: &CacheScope) {
        cache
            .internal
            .read_cache_file(Cache::Version, &mut |lines, path| {
                let content = cache
                    .insert_as_first_line(lines, self.installed_version.to_string())
                    .join("\n");
                if write(path, &content).is_err() {
                    self.on_record_version_use_error(content, path.to_owned());
                }
            });
    }

    fn hop_to_installed_version(&self, files: &FileSystem, cache: &CacheScope) {
        Logger::info(
            format!(
                "Switching to version {}",
                Logger::with_theme(|theme| theme.highlight(&self.installed_version))
            )
            .as_str(),
        );
        if RepoKitVersionResolver::hop_to_installed_version(files).is_ok() {
            self.record_version_use(cache);
        } else {
            Logger::info("I was unable swap to your currently installed version");
            Logger::info("Please file a bug at");
            Logger::log_issue_link();
        }
        panic!();
    }

    async fn runtime_version(&self, cache: &CacheScope) -> Option<String> {
        let runtime_version =
            cache
                .internal
                .read_cache_file(Cache::Version, &mut |mut lines, _| {
                    let last_known_version = CacheScope::unwrap_line(lines.nth(0), "");
                    if VERSION_REGEX.is_match(&last_known_version) {
                        return Some(last_known_version);
                    }
                    None
                });
        runtime_version.unwrap()
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

    fn on_record_version_use_error(&self, version: String, cache_directory: PathBuf) {
        PostProcessor::get().register_task(move || {
            Logger::info(
                "I attempted to write to cache file on disk, but the operation was unsuccessful",
            );
            CacheScope::log_cache_write_error();
            Logger::info("To avoid any issue with stale caches you can run");
            Logger::log_file_path(
                format!(
                    "printf \"{}\" > {}",
                    &version,
                    FileSystem::path_buf_to_str(&cache_directory),
                )
                .as_str(),
            );
        });
    }
}
