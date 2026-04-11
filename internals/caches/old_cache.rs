use std::{env::home_dir, path::PathBuf, sync::LazyLock};

use crate::caches::{file_cache::FileCache, version_cache::VERSION_REGEX};

static OLD_CACHE_FILE: &str = ".repokit";

static OLD_CACHE_FILE_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(home_dir);

pub struct OldCache;

impl FileCache for OldCache {
    fn cache_file(&self) -> &str {
        OLD_CACHE_FILE
    }

    fn cache_directory(&self) -> &Option<PathBuf> {
        &OLD_CACHE_FILE_PATH
    }
}

impl OldCache {
    pub fn new() -> Self {
        OldCache {}
    }

    pub fn get_version(&self) -> Option<String> {
        if let Some((mut lines, _)) = self.read() {
            let version = self.unwrap_line(lines.nth(0), "");
            if VERSION_REGEX.is_match(&version) {
                return Some(version);
            }
        }
        None
    }
}
