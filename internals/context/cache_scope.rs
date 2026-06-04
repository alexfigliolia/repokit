use std::{env::home_dir, fs::create_dir_all, path::PathBuf};
use tokio::{join, runtime::Runtime, task::JoinHandle};

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
    pub async fn new(git_scope: &GitScope, runtime: &Runtime) -> Self {
        let home = home_dir();
        let cache_directory =
            CacheScope::resolve_cache_directory(&home, &git_scope.root_commit_hash);
        let (crawl_cache, settings_cache) = join!(
            CacheScope::crawl_cache_thread(&cache_directory, git_scope, runtime),
            CacheScope::settings_cache_thread(&cache_directory, runtime),
        );
        CacheScope {
            crawl_cache: crawl_cache.unwrap(),
            settings_cache: settings_cache.unwrap(),
        }
    }

    fn crawl_cache_thread(
        cache_directory: &Option<PathBuf>,
        git_scope: &GitScope,
        runtime: &Runtime,
    ) -> JoinHandle<CrawlCache> {
        let dir_clone = cache_directory.clone();
        let scope_clone = git_scope.clone();
        runtime.spawn(CrawlCache::spawn(dir_clone, scope_clone))
    }

    fn settings_cache_thread(
        cache_directory: &Option<PathBuf>,
        runtime: &Runtime,
    ) -> JoinHandle<SettingsCache> {
        let dir_clone = cache_directory.clone();
        runtime.spawn(SettingsCache::spawn(dir_clone, ()))
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
