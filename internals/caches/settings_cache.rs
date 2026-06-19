use std::{
    fs::{read_to_string, remove_file},
    path::PathBuf,
};

use normalize_path::NormalizePath;

use crate::{
    caches::file_cache::FileCache, logger::logger::Logger,
    post_processing::post_processor::PostProcessor, themes::theme_registry::DEFAULT_THEME_NAME,
};

#[derive(Clone)]
pub struct SettingsCache {
    pub cache_directory: Option<PathBuf>,
    pub theme_preference: String,
}

impl SettingsCache {
    pub fn store_theme_preference(&self, theme: &str) {
        self.write(format!("{theme}\n").as_str(), |_| {
            self.on_theme_storage_error(theme);
        });
    }

    fn on_theme_storage_error(&self, theme: &str) {
        if let Some(storage_path) = self.storage_path() {
            let theme_clone = theme.to_string();
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
                        "printf \"{}\n\" > {}",
                        theme_clone,
                        storage_path.to_string_lossy(),
                    )
                    .as_str(),
                );
            });
        }
    }

    fn migrate(&self, old_cache_dir: Option<PathBuf>) -> Option<String> {
        if let Some(cache_dir) = old_cache_dir {
            let file_path = cache_dir.join(self.cache_file()).normalize();
            if file_path.exists()
                && let Ok(contents) = read_to_string(&file_path)
            {
                let _ = remove_file(&file_path);
                return Some(contents);
            }
        }
        None
    }

    fn read_theme_preference(&mut self) {
        if let Some((mut lines, _)) = self.read()
            && let Some(result) = lines.nth(0)
            && let Ok(theme) = result
        {
            self.theme_preference = theme;
        }
    }
}

impl FileCache<Option<PathBuf>> for SettingsCache {
    fn cache_file(&self) -> &str {
        ".settings"
    }

    fn cache_directory(&self) -> &Option<PathBuf> {
        &self.cache_directory
    }

    fn default_cache_contents(&self) -> &str {
        DEFAULT_THEME_NAME
    }

    fn creator(cache_directory: Option<PathBuf>) -> Self {
        SettingsCache {
            cache_directory: cache_directory.clone(),
            theme_preference: Logger::with_registry(|registry| registry.default_theme.to_owned()),
        }
    }

    fn initialize(&mut self, old_cache_dir: Option<PathBuf>) {
        if let Some(settings) = self.migrate(old_cache_dir)
            && self.write(&settings, |_| {}).is_none()
        {
            self.theme_preference = settings.trim().to_owned();
            return;
        }
        self.read_theme_preference();
    }
}
