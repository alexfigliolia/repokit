use std::{
    env::{args, current_dir},
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use regex::Regex;

use crate::{
    context::file_system::INSTALLED_PACKAGE_PATH, executor::executor::Executor,
    file_walker::upward_walker::UpwardWalker, internal_commands::list_version::REPOKIT_VERSION,
    logger::logger::Logger, post_processing::post_processor::PostProcessor,
};

pub static PACKAGE_VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\s*?"version":\s*?"(\d.\d.\d)""#).unwrap());

#[derive(Clone)]
pub struct InstallationScope {
    pub install_path: PathBuf,
}

impl InstallationScope {
    pub fn new() -> Self {
        let install_path = InstallationScope::resolve_install_path();
        let desired_version = InstallationScope::resolve_installed_version(&install_path);
        if desired_version != REPOKIT_VERSION {
            Logger::info(
                format!(
                    "Switching to version {}",
                    Logger::with_theme(|theme| theme.highlight(&desired_version))
                )
                .as_str(),
            );
            let success = Executor::with_stdio(
                format!("cargo install repokit@{}", desired_version),
                |cmd| cmd,
            );
            if success {
                PostProcessor::get().register_task(move || {
                    Executor::with_stdio("repokit", |cmd| cmd.args(args()));
                });
            } else {
                Logger::info(
                    format!(
                        "Failed to install version {}",
                        Logger::with_theme(|theme| theme.highlight(&desired_version))
                    )
                    .as_str(),
                );
                Logger::info(format!(
                    "This could be caused by your locally installed version number being out of range, or a bug within {}",
                    Logger::with_theme(|theme| theme.highlight("repokit"))
                ).as_str());
                Logger::info("If you believe it to be the latter, please file a bug here:");
                Logger::log_issue_link();
            }
            panic!();
        }
        Self { install_path }
    }

    fn resolve_install_path() -> PathBuf {
        if let Ok(working_directory) = current_dir() {
            let walker = UpwardWalker::new(&working_directory);
            if let Some(installation_path) = walker.search_for(&INSTALLED_PACKAGE_PATH) {
                return installation_path;
            }
        }
        InstallationScope::register_install_error();
        panic!()
    }

    fn resolve_installed_version(installed_path: &Path) -> String {
        let package_path = installed_path.join(format!("{}/package.json", *INSTALLED_PACKAGE_PATH));
        if package_path.exists()
            && let Ok(file) = File::open(&package_path)
        {
            let line_buffer = BufReader::new(file).lines();
            for line_result in line_buffer {
                if let Ok(line) = line_result
                    && let Some(captures) = PACKAGE_VERSION_REGEX.captures(&line)
                    && let Some(version) = captures.get(1)
                {
                    return version.as_str().to_string();
                }
            }
        }
        REPOKIT_VERSION.to_string()
    }

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
