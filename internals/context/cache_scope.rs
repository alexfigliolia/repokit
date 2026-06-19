use sha2::{Digest, Sha256};
use std::{
    env::home_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use tokio::{join, runtime::Runtime, task::JoinHandle};

use crate::{
    caches::{crawl_cache::CrawlCache, file_cache::FileCache, settings_cache::SettingsCache},
    context::{git_scope::GitScope, installation_scope::InstallationScope},
};

#[derive(Clone)]
pub struct CacheScope {
    pub crawl_cache: CrawlCache,
    pub settings_cache: SettingsCache,
}

static CACHE_DIRECTORY: &str = ".repokit_cache";

impl CacheScope {
    pub async fn new(
        git_scope: &GitScope,
        installation_scope: &InstallationScope,
        runtime: &Runtime,
    ) -> Self {
        let home = home_dir();
        let cache_directory =
            CacheScope::resolve_cache_directory(&home, &git_scope.root_commit_hash);
        let (crawl_cache, settings_cache) = join!(
            CacheScope::crawl_cache_thread(&cache_directory, git_scope, runtime),
            CacheScope::settings_cache_thread(&home, installation_scope, &cache_directory, runtime),
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
        let scope_clone = git_scope.clone();
        let cache_dir_clone = cache_directory.clone();
        runtime.spawn(async move { CrawlCache::spawn(cache_dir_clone, scope_clone) })
    }

    fn settings_cache_thread(
        home: &Option<PathBuf>,
        installation_scope: &InstallationScope,
        old_cache_directory: &Option<PathBuf>,
        runtime: &Runtime,
    ) -> JoinHandle<SettingsCache> {
        let cache_key = CacheScope::encode_install_path(&installation_scope.install_path);
        let cache_directory = CacheScope::resolve_cache_directory(home, &Some(cache_key));
        let old_cache_clone = old_cache_directory.clone();
        runtime.spawn(async move { SettingsCache::spawn(cache_directory, old_cache_clone) })
    }

    fn resolve_cache_directory(
        home_path: &Option<PathBuf>,
        target_dir: &Option<String>,
    ) -> Option<PathBuf> {
        if let Some(home) = home_path
            && let Some(target) = target_dir
        {
            let cache_dir = home.join(CACHE_DIRECTORY).join(target);
            if !cache_dir.exists() && create_dir_all(&cache_dir).is_err() {
                return None;
            }
            return Some(cache_dir);
        }
        None
    }

    fn encode_install_path(path: &Path) -> String {
        let mut hasher = Sha256::new();
        let path_string = path.to_string_lossy();
        hasher.update(path_string.as_bytes());
        let result: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        result.to_lowercase()
    }
}
