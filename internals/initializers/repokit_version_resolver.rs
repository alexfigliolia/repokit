use std::{env::args, path::PathBuf, process::exit};

use normalize_path::NormalizePath;
use terminal_spinners::{BOUNCING_BALL, SpinnerBuilder};

use crate::{
    executor::executor::Executor, internal_filesystem::internal_filesystem::InternalFileSystem,
    logger::logger::Logger,
};

pub struct RepoKitVersionResolver;

impl RepoKitVersionResolver {
    pub fn hop_to_runtime_version(root: &str, target_version: &str) {
        let internal_fs = InternalFileSystem::new(root);
        let package_path = internal_fs.absolute(&internal_fs.package_directory());
        let install_path = package_path.join("installation/install.sh").normalize();
        if install_path.is_absolute() && install_path.exists() {
            Logger::info(
                format!(
                    "Switching to version {}",
                    Logger::with_theme(|theme| theme.highlight(target_version))
                )
                .as_str(),
            );
            if let Some(errors) = RepoKitVersionResolver::run_post_install(&package_path) {
                println!("{errors}");
            } else {
                RepoKitVersionResolver::re_run_command();
            }
        }
    }

    fn run_post_install(cwd: &PathBuf) -> Option<String> {
        let handle = SpinnerBuilder::new()
            .spinner(&BOUNCING_BALL)
            .text(" Installing")
            .start();
        let result =
            Executor::exec_with_errors("./installation/install.sh", |cmd| cmd.current_dir(cwd));
        handle.done();
        result
    }

    fn re_run_command() {
        let args: Vec<String> = args().collect();
        Executor::with_stdio(args.join(" "), |cmd| cmd);
        exit(0);
    }
}
