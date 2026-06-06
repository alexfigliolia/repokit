use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use regex::Regex;

use crate::{
    caches::file_cache::FileCache, context::git_scope::GitScope, executor::executor::Executor,
    logger::logger::Logger, post_processing::post_processor::PostProcessor,
};

#[derive(Clone)]
pub struct CrawlCache {
    pub cache_directory: Option<PathBuf>,
    pub changed_files: Option<Vec<String>>,
    pub files_to_crawl: Option<Vec<String>>,
}

impl CrawlCache {
    pub fn crawl_cache_enabled(&self) -> bool {
        self.files_to_crawl.is_some()
    }

    pub fn cache_crawl_results(&self, results: String) {
        self.write(&results, |_| {
            self.on_crawl_cache_storage_error();
        });
    }

    fn on_crawl_cache_storage_error(&self) {
        if let Some(storage_path) = self.storage_path() {
            PostProcessor::get().register_task(move || {
            Logger::info(
                "I attempted to cache the results of a file crawl, but couldn't write to disk",
            );
            CrawlCache::log_cache_write_error();
            Logger::info(
                "To avoid issues with stale caches I'm going to delete what's currently on disk",
            );
            CrawlCache::clear_cache_file(storage_path.clone(), true);
        });
        }
    }

    fn get_changed_files(&self, git_root: &Path) -> (bool, HashSet<String>) {
        let mut contains_git_ignore = false;
        let file_path_matcher = Regex::new(r#"^.*\s(.*\.ts)$"#).unwrap();
        let stdout = Executor::exec("git status --porcelain -uall", |cmd| cmd);
        let files: HashSet<String> = stdout
            .split("\n")
            .filter_map(|file| {
                if !contains_git_ignore && file.ends_with(".gitignore") {
                    contains_git_ignore = true;
                    return None;
                }
                let matches: Vec<&str> = file_path_matcher
                    .captures_iter(file)
                    .filter_map(|entry| {
                        if let Some(match_result) = entry.get(1) {
                            return Some(match_result.as_str());
                        }
                        None
                    })
                    .collect();
                if let Some(file_path) = matches.first()
                    && git_root.join(file_path).exists()
                {
                    return Some(file_path.to_string());
                }
                None
            })
            .collect();
        (contains_git_ignore, files)
    }
}

impl FileCache<GitScope> for CrawlCache {
    fn cache_file(&self) -> &str {
        ".crawl_cache"
    }

    fn cache_directory(&self) -> &Option<PathBuf> {
        &self.cache_directory
    }

    fn default_cache_contents(&self) -> &str {
        ""
    }

    fn creator(cache_directory: Option<PathBuf>) -> Self {
        CrawlCache {
            changed_files: None,
            files_to_crawl: None,
            cache_directory: cache_directory.clone(),
        }
    }

    fn initialize(&mut self, git_scope: GitScope) {
        if let Some(head_commit) = &git_scope.head_commit_hash
            && let Some((mut lines, path)) = self.read()
        {
            if &self.unwrap_line(lines.nth(0), "non_existent_hash") != head_commit {
                return;
            }
            let lines = self.line_buffer_to_vec(lines);
            if lines.is_empty() {
                return;
            }
            if let Some(git_root) = &git_scope.root_path {
                let (git_ignore_changed, mut changed_files) = self.get_changed_files(git_root);
                if git_ignore_changed {
                    CrawlCache::clear_cache_file(path.to_owned(), false);
                    return;
                }
                for line in &lines {
                    if changed_files.contains(line) {
                        changed_files.remove(line);
                    }
                }
                if !changed_files.is_empty() {
                    self.changed_files = Some(changed_files.into_iter().collect());
                }
                self.files_to_crawl = Some(lines);
            }
        }
    }
}
