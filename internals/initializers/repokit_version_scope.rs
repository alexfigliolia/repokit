use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use normalize_path::NormalizePath;
use regex::Regex;
use tokio_thread_pool::ThreadPool;

use crate::{
    executor::executor::Executor,
    initializers::{
        initializer::Initializer, internal_caches::InternalCaches,
        repokit_version_resolver::RepoKitVersionResolver,
    },
    internal_filesystem::internal_filesystem::{InternalFileSystem, VERSION_REGEX},
};

static UNKNOWN: &str = "unknown";

#[derive(Clone)]
pub struct RepoKitVersionScope {
    pub runtime_version: String,
    pub installed_version: String,
}

impl Initializer<()> for RepoKitVersionScope {
    async fn resolve(&mut self, root: &str) {
        let root_clone = root.to_string();
        let mut pool = ThreadPool::new(None, None, None);
        let runtime_handle = pool.spawn(RepoKitVersionScope::runtime_version);
        let install_handle =
            pool.spawn(move || RepoKitVersionScope::installed_repokit_version(&root_clone));
        self.runtime_version = runtime_handle.await.unwrap().unwrap_or(UNKNOWN.to_string());
        self.installed_version = install_handle.await.unwrap().unwrap_or(UNKNOWN.to_string());
    }
}

impl RepoKitVersionScope {
    pub fn new(root: &str) -> RepoKitVersionScope {
        let mut instance = RepoKitVersionScope {
            runtime_version: UNKNOWN.to_string(),
            installed_version: UNKNOWN.to_string(),
        };
        RepoKitVersionScope::resolve_sync(instance.resolve(root));
        instance.hop_to_runtime_version(root);
        instance
    }

    pub fn refresh_installed_version(&self, root: &str) -> Option<String> {
        let installed_version = RepoKitVersionScope::installed_repokit_version(root);
        if let Some(version) = &installed_version
            && *version != self.installed_version
        {
            return Some(version.to_string());
        }
        None
    }

    fn hop_to_runtime_version(&self, root: &str) {
        if self.runtime_version != self.installed_version {
            RepoKitVersionResolver::hop_to_runtime_version(root, &self.installed_version);
        }
    }

    fn runtime_version() -> Option<String> {
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

    pub fn installed_repokit_version(root: &str) -> Option<String> {
        let internal_fs = InternalFileSystem::new(root);
        let package_path = Path::new(&root)
            .join(internal_fs.package_directory())
            .join("package.json")
            .normalize();
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
