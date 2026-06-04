use std::{
    fs::{File, write},
    io::{BufRead, BufReader, Error, Lines},
    path::PathBuf,
};

use normalize_path::NormalizePath;

use crate::{logger::logger::Logger, post_processing::post_processor::PostProcessor};

pub trait FileCache<T> {
    async fn spawn(cache_directory: Option<PathBuf>, options: T) -> Self
    where
        Self: Sized,
    {
        let mut instance: Self = FileCache::creator(cache_directory);
        instance.create_cache_file_if_not_exists().await;
        instance.initialize(options).await;
        instance
    }

    fn creator(options: Option<PathBuf>) -> Self;

    fn cache_file(&self) -> &str;

    fn cache_directory(&self) -> &Option<PathBuf>;

    fn default_cache_contents(&self) -> &str;

    async fn initialize(&mut self, options: T);

    fn storage_path(&self) -> Option<PathBuf> {
        if let Some(storage_path) = self.perspective_path()
            && storage_path.exists()
        {
            return Some(storage_path);
        }
        None
    }

    fn perspective_path(&self) -> Option<PathBuf> {
        if let Some(cache_dir) = &self.cache_directory() {
            return Some(cache_dir.join(self.cache_file()).normalize());
        }
        None
    }

    async fn create_cache_file_if_not_exists(&self) -> Option<PathBuf> {
        if let Some(storage_path) = self.perspective_path()
            && !&storage_path.exists()
            && write(&storage_path, self.default_cache_contents()).is_ok()
        {
            return Some(storage_path);
        }
        None
    }

    fn read(&self) -> Option<(Lines<BufReader<File>>, PathBuf)> {
        if let Some(path) = self.storage_path()
            && let Ok(file) = File::open(&path)
        {
            return Some((BufReader::new(file).lines(), path));
        }
        None
    }

    fn write(&self, content: &str, on_error: impl Fn(Error)) {
        if let Some(path) = self.storage_path()
            && let Err(error) = write(path, content)
        {
            on_error(error);
        }
    }

    fn line_buffer_to_vec(&self, lines: Lines<BufReader<File>>) -> Vec<String> {
        lines.map_while(Result::ok).collect()
    }

    fn insert_as_first_line(&self, value: String, on_error: impl Fn(Error)) {
        if let Some((lines, _)) = self.read() {
            let mut vec: Vec<String> = self.line_buffer_to_vec(lines);
            if !vec.is_empty() {
                vec[0] = value;
            } else {
                vec.push(value);
            }
            self.write(vec.join("\n").as_str(), on_error);
        }
        // TODO handle write error with telemetry regarding git scope
    }

    fn unwrap_line(&self, line_result: Option<Result<String, Error>>, fallback: &str) -> String {
        line_result
            .and_then(|r| r.ok())
            .unwrap_or(fallback.to_string())
    }

    fn clear_cache_file(path: PathBuf, notify: bool) {
        PostProcessor::get().register_task(move || {
            if write(&path, "").is_err() {
                Logger::error("I was unable to remove a cache on disk");
                Logger::error("To correct this, please run");
                Logger::log_file_path(format!("rm {}", path.to_string_lossy()).as_str())
            } else if notify {
                Logger::info("Cache deleted!");
            }
        });
    }

    fn log_cache_write_error() {
        Logger::info("This is typically a permission-related issue on the operating system");
        Logger::info(
            format!(
                "If you believe this to be a bug within {}",
                Logger::with_theme(|theme| theme.highlight("Repokit"))
            )
            .as_str(),
        );
        Logger::info("Please file an issue here");
        Logger::log_issue_link();
    }
}
