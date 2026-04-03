use std::{
    fs::{File, write},
    io::{BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};

use normalize_path::NormalizePath;
use shellexpand::tilde;

use crate::logger::logger::Logger;

pub enum Cache {
    SETTINGS,
    FILESYSTEM,
}

#[derive(Clone)]
pub struct InternalCaches {
    pub settings_path: Option<PathBuf>,
    pub filesystem_cache_path: Option<PathBuf>,
}

impl InternalCaches {
    pub fn new(version: &str) -> InternalCaches {
        let mut instance = InternalCaches::blank();
        instance.create_settings_cache(version);
        instance.create_filesystem_cache();
        instance
    }

    fn blank() -> InternalCaches {
        InternalCaches {
            settings_path: None,
            filesystem_cache_path: None,
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
            Cache::SETTINGS => InternalCaches::try_read(&self.settings_path, func),
            Cache::FILESYSTEM => InternalCaches::try_read(&self.filesystem_cache_path, func),
        }
    }

    fn try_read<R>(
        path_buf: &Option<PathBuf>,
        func: &mut impl FnMut(Lines<BufReader<File>>, &PathBuf) -> R,
    ) -> Option<R> {
        if let Some(path) = path_buf
            && let Ok(file) = File::open(path)
        {
            let lines = BufReader::new(file).lines();
            return Some(func(lines, path));
        }
        None
    }

    fn create_settings_cache(&mut self, installed_version: &str) {
        let mut created = false;
        if let Some(path) = InternalCaches::create_cache_file_if_not_exists(".repokit", &mut |_| {
            created = true;
            installed_version.to_string()
        }) {
            self.settings_path = Some(path);
        }
    }

    fn create_filesystem_cache(&mut self) {
        if let Some(path) =
            InternalCaches::create_cache_file_if_not_exists(".repokit_cache", &mut |_| {
                "unknown_last_commit".to_string()
            })
        {
            self.filesystem_cache_path = Some(path);
        }
    }

    fn create_cache_file_if_not_exists(
        file_name: &str,
        on_create: &mut impl FnMut(&PathBuf) -> String,
    ) -> Option<PathBuf> {
        match InternalCaches::home() {
            Some(home) => {
                let dot_file_path = home.join(file_name);
                if !&dot_file_path.exists() {
                    let contents = on_create(&dot_file_path);
                    let result = write(&dot_file_path, format!("{}\n", contents));
                    if result.is_err() {
                        return None;
                    }
                }
                Some(dot_file_path)
            }
            None => {
                Logger::error(
                    "I encountered an issue when attempting to create a cache file on your machine",
                );
                Logger::error(
                    format!(
                        "Please create a file called {} in your home directory",
                        Logger::with_theme(|theme| theme.highlight(file_name))
                    )
                    .as_str(),
                );
                Logger::error(
                    "This file will be used to store settings and indicators that optimize how repokit runs in your repository",
                );
                None
            }
        }
    }
}
