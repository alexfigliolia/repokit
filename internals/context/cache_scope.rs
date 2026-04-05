use std::{
    collections::HashSet,
    fs::{File, write},
    io::{self, BufReader, Lines},
    path::PathBuf,
};

use futures::join;
use regex::Regex;

use crate::{
    context::{
        file_system::FileSystem,
        git_scope::GitScope,
        initializer::Initializer,
        internal_caches::{Cache, InternalCaches},
    },
    executor::executor::Executor,
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

#[derive(Clone)]
pub struct CacheScope {
    pub internal: InternalCaches,
    pub theme_preference: String,
    pub files_to_crawl: Option<Vec<String>>,
}

impl Initializer<(), &str> for CacheScope {
    async fn resolve(&mut self, head_commit: &str) {
        let (theme, file_cache) = join!(
            self.read_theme_preference(),
            self.read_file_system_cache(head_commit)
        );
        self.theme_preference = theme;
        self.files_to_crawl = file_cache;
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
        CacheScope::resolve_sync(instance.resolve(head_commit.unwrap_or("".to_string()).as_str()));
        instance
    }

    pub fn crawl_cache_enabled(&self) -> bool {
        self.files_to_crawl.is_some()
    }

    pub fn store_theme_preference(&self, theme: &str) {
        self.internal
            .read_cache_file(Cache::Settings, &mut |lines, path| {
                let content = self
                    .insert_as_first_line(lines, theme.to_string())
                    .join("\n");
                if write(path, &content).is_err() {
                    self.on_theme_storage_error(path, content);
                }
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

    pub fn insert_as_first_line(
        &self,
        lines: Lines<BufReader<File>>,
        value: String,
    ) -> Vec<String> {
        let mut lines: Vec<String> = self.line_buffer_to_vec(lines);
        if !lines.is_empty() {
            lines[0] = value;
        } else {
            lines.push(value);
        }
        lines
    }

    pub fn line_buffer_to_vec(&self, lines: Lines<BufReader<File>>) -> Vec<String> {
        let vec: Vec<String> = lines
            .filter_map(|entry| {
                if let Ok(line) = entry {
                    Some(line);
                }
                None
            })
            .collect();
        vec
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
            .read_cache_file(Cache::FileSystem, &mut move |line_buffer, _| {
                let mut lines = self.line_buffer_to_vec(line_buffer);
                if lines.len() < 2 {
                    return None;
                }
                if let Some(crawl_commit) = lines.first()
                    && crawl_commit == head_commit
                {
                    let (git_ignore_changed, mut changed_files) = CacheScope::get_changed_files();
                    if git_ignore_changed {
                        self.drop_file_system_cache();
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

    fn drop_file_system_cache(&self) {
        self.internal
            .read_cache_file(Cache::FileSystem, &mut |mut lines, path| {
                if let Some(first_line) = lines.nth(0)
                    && let Ok(head_commit) = first_line
                {
                    // TODO - error handle
                    write(path, format!("{head_commit}\n"))
                } else {
                    write(path, "")
                }
            });
    }

    fn on_theme_storage_error(&self, path: &PathBuf, content: String) {
        let path_clone = path.clone();
        PostProcessor::get().register_task(move || {
            Logger::info("I failed to write your theme preference to the file system");
            Logger::info(
                format!(
                    "This usually indicates a bug within {}",
                    Logger::with_theme(|theme| theme.highlight("Repokit"))
                )
                .as_str(),
            );
            Logger::info("Please file an issue here");
            Logger::log_issue_link();
            Logger::info("To repair this manually you can run");
            Logger::log_file_path(
                format!(
                    "printf \"{}\" > {}",
                    &content,
                    FileSystem::path_buf_to_str(&path_clone),
                )
                .as_str(),
            );
        });
    }
}
