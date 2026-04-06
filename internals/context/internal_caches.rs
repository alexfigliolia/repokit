use std::{
    fs::{File, create_dir_all, write},
    io::{BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use futures::join;
use normalize_path::NormalizePath;
use shellexpand::tilde;

use crate::context::{git_scope::GitScope, initializer::Initializer};

pub enum Cache {
    Version,
    Settings,
    FileSystem,
}

#[derive(Clone)]
pub struct InternalCaches {
    pub version_path: Option<PathBuf>,
    pub settings_path: Option<PathBuf>,
    pub cache_directory: Option<PathBuf>,
    pub filesystem_cache_path: Option<PathBuf>,
}

impl Initializer<(), &str> for InternalCaches {
    async fn resolve(&mut self, _: &str) {
        let (settings_path, files_path, version_path) = join!(
            self.create_settings_cache(),
            self.create_filesystem_cache(),
            self.create_version_cache()
        );
        self.settings_path = settings_path;
        self.filesystem_cache_path = files_path;
        self.version_path = version_path;
    }
}

impl InternalCaches {
    pub fn new(git_scope: &GitScope) -> InternalCaches {
        let mut instance = InternalCaches::blank(&git_scope.root_commit_hash);
        if instance.cache_directory.is_some() {
            InternalCaches::resolve_sync(instance.resolve(""));
        }
        instance
    }

    fn blank(root_commit: &Option<String>) -> InternalCaches {
        InternalCaches {
            version_path: None,
            settings_path: None,
            filesystem_cache_path: None,
            cache_directory: InternalCaches::resolve_cache_directory(root_commit),
        }
    }

    pub fn home() -> Option<PathBuf> {
        let expanded_path_str = tilde("~/");
        let path = Path::new(expanded_path_str.as_ref()).normalize();
        if path.is_absolute() && path.exists() {
            return Some(path);
        }
        None
    }

    pub fn read_cache_file<R>(
        &self,
        cache: Cache,
        func: &mut impl FnMut(Lines<BufReader<File>>, &PathBuf) -> R,
    ) -> Option<R> {
        match cache {
            Cache::Version => InternalCaches::try_read(&self.version_path, func),
            Cache::Settings => InternalCaches::try_read(&self.settings_path, func),
            Cache::FileSystem => InternalCaches::try_read(&self.filesystem_cache_path, func),
        }
    }

    fn resolve_cache_directory(root_commit: &Option<String>) -> Option<PathBuf> {
        if let Some(home) = InternalCaches::home()
            && let Some(commit_hash) = root_commit
        {
            let cache_dir = home.join(".repokit").join(commit_hash);
            if !cache_dir.exists() {
                let created = create_dir_all(&cache_dir);
                if created.is_err() {
                    return None;
                }
            }
            return Some(cache_dir);
        }
        None
    }

    fn try_read<R>(
        path_buf: &Option<PathBuf>,
        func: &mut impl FnMut(Lines<BufReader<File>>, &PathBuf) -> R,
    ) -> Option<R> {
        if let Some(path) = path_buf
            && let Ok(file) = File::open(path)
        {
            return Some(func(BufReader::new(file).lines(), path));
        }
        None
    }

    async fn create_settings_cache(&self) -> Option<PathBuf> {
        if let Some(path) =
            self.create_cache_file_if_not_exists(".repokit_settings", &mut |_| "".to_string())
        {
            return Some(path);
        }
        None
    }

    async fn create_filesystem_cache(&self) -> Option<PathBuf> {
        if let Some(path) =
            self.create_cache_file_if_not_exists(".repokit_cache", &mut |_| "".to_string())
        {
            return Some(path);
        }
        None
    }

    async fn create_version_cache(&self) -> Option<PathBuf> {
        if let Some(path) =
            self.create_cache_file_if_not_exists("../.repokit_version", &mut |_| "".to_string())
        {
            return Some(path);
        }
        None
    }

    fn create_cache_file_if_not_exists(
        &self,
        file_name: &str,
        on_create: &mut impl FnMut(&PathBuf) -> String,
    ) -> Option<PathBuf> {
        if let Some(cache_directory) = &self.cache_directory {
            let dot_file_path = cache_directory.join(file_name).normalize();
            if !&dot_file_path.exists() {
                let contents = on_create(&dot_file_path);
                let result = write(&dot_file_path, format!("{}\n", contents));
                if result.is_err() {
                    return None;
                }
            }
            return Some(dot_file_path);
        }
        None
    }
}
