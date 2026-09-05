use std::path::PathBuf;

use crate::{
    caches::file_cache::FileCache, logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

#[derive(Clone)]
pub struct GitNotifierCache {
    pub cache_directory: Option<PathBuf>,
    pub notification_state: bool,
}

impl GitNotifierCache {
    pub fn notify(&self) {
        self.store_notifier();
        PostProcessor::get().register_task(|| {
            Logger::info(format!(
            "Running {} in your workspace will allow {} to cache file crawls more aggressively and improve performance",
            Logger::with_theme(|theme| theme.highlight("git init")),
            Logger::with_theme(|theme| theme.highlight("Repokit"))).as_str());
        });
    }

    fn store_notifier(&self) {
        self.write("true\n".to_string().as_str(), |_| {
            self.on_git_notificer_storage_error();
        });
    }

    fn on_git_notificer_storage_error(&self) {
        if let Some(storage_path) = self.storage_path() {
            PostProcessor::get().register_task(move || {
                Logger::info("I failed to write to a cache file in your file system");
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
                    format!("printf \"true\n\" > {}", storage_path.to_string_lossy(),).as_str(),
                );
            });
        }
    }
}

impl FileCache<()> for GitNotifierCache {
    fn cache_file(&self) -> &str {
        ".git_cache_notification"
    }

    fn cache_directory(&self) -> &Option<PathBuf> {
        &self.cache_directory
    }

    fn default_cache_contents(&self) -> &str {
        "false"
    }

    fn creator(cache_directory: Option<PathBuf>) -> Self {
        GitNotifierCache {
            cache_directory: cache_directory.clone(),
            notification_state: false,
        }
    }

    fn initialize(&mut self, _options: ()) {
        if let Some((mut lines, _)) = self.read()
            && let Some(result) = lines.nth(0)
            && let Ok(status) = result
        {
            let as_boolean: bool = status.parse().unwrap_or(false);
            self.notification_state = as_boolean;
        }
    }
}
