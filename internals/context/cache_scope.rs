use futures::{executor::block_on, join};
use normalize_path::NormalizePath;
use shellexpand::tilde;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::{
    caches::{
        crawl_cache::CrawlCache, file_cache::FileCache, settings_cache::SettingsCache,
        version_cache::VersionCache,
    },
    context::{file_system::FileSystem, git_scope::GitScope},
};

#[derive(Clone)]
pub struct CacheScope {
    pub version_cache: VersionCache,
    pub settings_cache: SettingsCache,
    pub crawl_cache: CrawlCache,
}

static CACHE_DIRECTORY: &str = ".repokit";

impl CacheScope {
    pub fn new(git_scope: &GitScope, file_system: &FileSystem) -> CacheScope {
        let cache_directory = CacheScope::resolve_cache_directory(&git_scope.root_commit_hash);
        let mut instance = CacheScope {
            crawl_cache: CrawlCache::new(&cache_directory),
            version_cache: VersionCache::new(&cache_directory),
            settings_cache: SettingsCache::new(&cache_directory),
        };
        block_on(instance.initialize_all(git_scope, file_system));
        instance
    }

    async fn initialize_all(&mut self, git_scope: &GitScope, file_system: &FileSystem) {
        self.create_cache_files().await;
        join!(
            self.version_cache.initialize(file_system),
            self.settings_cache.initialize(),
            self.crawl_cache.initialize(git_scope),
        );
    }

    async fn create_cache_files(&self) {
        join!(
            self.version_cache.create_cache_file_if_not_exists(),
            self.settings_cache.create_cache_file_if_not_exists(),
            self.crawl_cache.create_cache_file_if_not_exists(),
        );
    }

    pub fn home() -> Option<PathBuf> {
        let expanded_path_str = tilde("~/");
        let path = Path::new(expanded_path_str.as_ref()).normalize();
        if path.is_absolute() && path.exists() {
            return Some(path);
        }
        None
    }

    fn resolve_cache_directory(root_commit: &Option<String>) -> Option<PathBuf> {
        if let Some(home) = CacheScope::home()
            && let Some(commit_hash) = root_commit
        {
            let cache_dir = home.join(CACHE_DIRECTORY).join(commit_hash);
            if !cache_dir.exists()
                && create_dir_all(&cache_dir).is_err() {
                    return None;
                }
            return Some(cache_dir);
        }
        None
    }
}
