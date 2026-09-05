use sha2::{Digest, Sha256};
use std::{
    env::home_dir,
    fs::{create_dir_all, remove_dir_all, rename},
    path::{Path, PathBuf},
};
use tokio::{join, runtime::Runtime, task::JoinHandle};

use crate::{
    caches::{
        crawl_cache::CrawlCache, file_cache::FileCache, git_notifier_cache::GitNotifierCache,
        settings_cache::SettingsCache,
    },
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
        let cache_directory = CacheScope::resolve_cache_path(git_scope, installation_scope);
        let (crawl_cache, settings_cache, git_notifier_cache) = join!(
            CacheScope::crawl_cache_thread(&cache_directory, git_scope, runtime),
            CacheScope::settings_cache_thread(&cache_directory, runtime),
            CacheScope::git_notify_cache(&cache_directory, runtime),
        );
        let git_notifier = git_notifier_cache.unwrap();
        if !git_notifier.notification_state && git_scope.root_commit_hash.is_none() {
            git_notifier.notify()
        }
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
        cache_directory: &Option<PathBuf>,
        runtime: &Runtime,
    ) -> JoinHandle<SettingsCache> {
        let clone = cache_directory.clone();
        runtime.spawn(async move { SettingsCache::spawn(clone, ()) })
    }

    fn git_notify_cache(
        cache_directory: &Option<PathBuf>,
        runtime: &Runtime,
    ) -> JoinHandle<GitNotifierCache> {
        let clone = cache_directory.clone();
        runtime.spawn(async move { GitNotifierCache::spawn(clone, ()) })
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

    fn resolve_cache_path(
        git_scope: &GitScope,
        installation_scope: &InstallationScope,
    ) -> Option<PathBuf> {
        if let Some(home) = home_dir() {
            let encoded_install_path =
                CacheScope::encode_install_path(&installation_scope.install_path);
            let root_cache_dir = home.join(CACHE_DIRECTORY);
            let fallback_path = root_cache_dir.join(encoded_install_path);
            let fallback_path_exists = fallback_path.exists();
            if let Some(root_commit) = &git_scope.root_commit_hash {
                let git_root_cache_path = root_cache_dir.join(root_commit); // this is a huge mess.
                let git_path_exists = &git_root_cache_path.exists();
                if fallback_path_exists {
                    if !git_path_exists {
                        if rename(&fallback_path, &git_root_cache_path).is_ok() {
                            return Some(git_root_cache_path);
                        }
                        return Some(fallback_path);
                    }
                    let _ = remove_dir_all(&fallback_path);
                }
                if !git_path_exists {
                    let _ = create_dir_all(&git_root_cache_path);
                }
                if git_root_cache_path.exists() {
                    return Some(git_root_cache_path);
                }
            }
            if !fallback_path_exists {
                let _ = create_dir_all(&fallback_path);
            }
            if fallback_path.exists() {
                return Some(fallback_path);
            }
        }
        None
    }
}
