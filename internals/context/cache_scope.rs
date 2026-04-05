use futures::join;
use std::{collections::HashSet, fs::write, io};

use regex::Regex;

use crate::{
    context::{
        git_scope::GitScope,
        initializer::Initializer,
        internal_caches::{Cache, InternalCaches},
    },
    executor::executor::Executor,
    logger::logger::Logger,
};

#[derive(Clone)]
pub struct CacheScope {
    pub internal: InternalCaches,
    pub theme_preference: String,
    pub files_to_crawl: Option<Vec<String>>,
}

impl Initializer<(String, Option<Vec<String>>), &str> for CacheScope {
    async fn resolve(&mut self, head_commit: &str) -> (String, Option<Vec<String>>) {
        join!(
            self.read_theme_preference(),
            self.read_file_system_cache(head_commit)
        )
    }
}

impl CacheScope {
    pub fn new(git_scope: &GitScope) -> CacheScope {
        let mut instance = CacheScope {
            files_to_crawl: None,
            theme_preference: "".to_string(),
            internal: InternalCaches::new(git_scope),
        };
        let head_commit = git_scope.head_commit_hash.clone();
        let (theme, file_cache) = CacheScope::resolve_sync(
            instance.resolve(head_commit.unwrap_or("".to_string()).as_str()),
        );
        instance.theme_preference = theme;
        instance.files_to_crawl = file_cache;
        instance
    }

    pub fn crawl_cache_enabled(&self) -> bool {
        self.files_to_crawl.is_some()
    }

    pub fn store_theme_preference(&self, theme: &str) {
        self.internal
            .read_cache_file(Cache::Settings, &mut |lines, path| {
                let mut content: Vec<String> = lines.map(|line| line.unwrap()).collect();
                let theme_text = theme.to_string();
                if !content.is_empty() {
                    content[0] = theme_text;
                } else {
                    content.push(theme_text);
                }
                write(path, content.join("\n"))
            });
    }

    pub fn store_crawl_cache(
        &self,
        head_commit: &str,
        paths: String,
    ) -> Option<Result<(), io::Error>> {
        self.internal
            .read_cache_file(Cache::FileSystem, &mut move |_, file_path| {
                write(file_path, [head_commit, &paths].join("\n"))
            })
    }

    async fn read_theme_preference(&self) -> String {
        let default = Logger::with_registry(|registry| registry.default_theme.clone());
        let theme = self
            .internal
            .read_cache_file(Cache::Settings, &mut |mut lines, _| {
                if let Some(theme_preference) = lines.nth(0)
                    && let Ok(preference) = theme_preference
                {
                    return preference;
                }
                default.to_string()
            });
        theme.unwrap_or(default)
    }

    async fn read_file_system_cache(&self, head_commit: &str) -> Option<Vec<String>> {
        let files_to_crawl = self
            .internal
            .read_cache_file(Cache::FileSystem, &mut move |lines, _| {
                let mut lines: Vec<String> = lines
                    .filter_map(|entry| {
                        if let Ok(line) = entry {
                            return Some(line);
                        }
                        None
                    })
                    .collect();
                if lines.len() < 2 {
                    return None;
                }
                if let Some(crawl_commit) = lines.first()
                    && crawl_commit == head_commit
                {
                    let (git_ignore_changed, mut changed_files) = CacheScope::get_changed_files();
                    if git_ignore_changed {
                        return None;
                    }
                    for line in &lines {
                        if changed_files.contains(line) {
                            changed_files.remove(line);
                        }
                    }
                    for line in changed_files {
                        lines.push(line);
                    }
                    return Some(lines);
                }
                None
            })
            .unwrap();
        if let Some(files) = files_to_crawl
            && files.len() > 1
        {
            return Some((files)[1..].to_vec());
        }
        None
    }

    fn get_changed_files() -> (bool, HashSet<String>) {
        let mut contains_git_ignore = false;
        let file_path_matcher = Regex::new(r#"^.*\s(.*\.ts)$"#).unwrap();
        let stdout = Executor::exec("git status --porcelain", |cmd| cmd);
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
                if let Some(file_path) = matches.first() {
                    return Some(file_path.to_string());
                }
                None
            })
            .collect();
        (contains_git_ignore, files)
    }
}
