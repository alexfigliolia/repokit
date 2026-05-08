use futures::{executor::block_on, join};
use std::{env::home_dir, fs::create_dir_all, path::PathBuf};

use crate::{
    caches::{crawl_cache::CrawlCache, file_cache::FileCache, settings_cache::SettingsCache},
    context::git_scope::GitScope,
};

#[derive(Clone)]
pub struct CacheScope {
    pub crawl_cache: CrawlCache,
    pub settings_cache: SettingsCache,
}

static CACHE_DIRECTORY: &str = ".repokit_cache";

impl CacheScope {
    pub fn new(git_scope: &GitScope) -> CacheScope {
        let home = home_dir();
        let cache_directory =
            CacheScope::resolve_cache_directory(&home, &git_scope.root_commit_hash);
        let mut instance = CacheScope {
            crawl_cache: CrawlCache::new(&cache_directory),
            settings_cache: SettingsCache::new(&cache_directory),
        };
        block_on(instance.initialize_all(git_scope));
        instance
    }

    async fn initialize_all(&mut self, git_scope: &GitScope) {
        self.create_cache_files().await;
        join!(
            self.settings_cache.initialize(),
            self.crawl_cache.initialize(git_scope),
        );
    }

    async fn create_cache_files(&self) {
        join!(
            self.settings_cache.create_cache_file_if_not_exists(),
            self.crawl_cache.create_cache_file_if_not_exists(),
        );
    }

    fn resolve_cache_directory(
        home_path: &Option<PathBuf>,
        root_commit: &Option<String>,
    ) -> Option<PathBuf> {
        if let Some(home) = home_path
            && let Some(commit_hash) = root_commit
        {
            let cache_dir = home.join(CACHE_DIRECTORY).join(commit_hash);
            if !cache_dir.exists() && create_dir_all(&cache_dir).is_err() {
                return None;
            }
            return Some(cache_dir);
        }
        None
    }
}
