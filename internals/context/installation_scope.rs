use std::{env::current_dir, path::PathBuf};

use crate::{
    context::{async_scope::AsyncScope, file_system::INSTALLED_PACKAGE_PATH},
    file_walker::upward_walker::UpwardWalker,
    logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

#[derive(Clone)]
pub struct InstallationScope {
    pub install_path: PathBuf,
}

impl AsyncScope<PathBuf> for InstallationScope {
    async fn new() -> Self {
        Self {
            install_path: InstallationScope::resolve().await,
        }
    }

    async fn resolve() -> PathBuf {
        if let Ok(working_directory) = current_dir() {
            let walker = UpwardWalker::new(&working_directory);
            if let Some(installation_path) = walker.search_for(&INSTALLED_PACKAGE_PATH) {
                return installation_path;
            }
        }
        InstallationScope::register_install_error();
        panic!()
    }
}

impl InstallationScope {
    fn register_install_error() {
        PostProcessor::get().register_task(|| {
            Logger::error(
                format!(
                    "I could not find your {} installation",
                    Logger::with_theme(|theme| theme.highlight("Repokit"))
                )
                .as_str(),
            );
            Logger::error(
                format!(
                    "Please make sure you execute the {} command from a working directory or subdirectory containing your {} installation",
                    Logger::with_theme(|theme| theme.highlight("repokit")),
                    Logger::with_theme(|theme| theme.highlight("Repokit"))
                )
                .as_str(),
            );
            Logger::error(
               format!("If you believe this to be a bug within {}, please file a bug here", Logger::with_theme(|theme| theme.highlight("Repokit"))).as_str()
            );
        });
    }
}
