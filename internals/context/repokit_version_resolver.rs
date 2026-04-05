use std::env::args;

use terminal_spinners::{BOUNCING_BALL, SpinnerBuilder};

use crate::{context::file_system::FileSystem, executor::executor::Executor};

pub struct RepoKitVersionResolver;

impl RepoKitVersionResolver {
    pub fn hop_to_installed_version(files: &FileSystem) -> bool {
        if files.install_script_path.is_absolute() && files.install_script_path.exists() {
            if let Some(errors) = RepoKitVersionResolver::run_post_install(files) {
                println!("{errors}");
            } else {
                RepoKitVersionResolver::re_run_command();
                return true;
            }
        }
        false
    }

    fn run_post_install(files: &FileSystem) -> Option<String> {
        let handle = SpinnerBuilder::new()
            .spinner(&BOUNCING_BALL)
            .text(" Installing")
            .start();
        let result =
            Executor::exec_with_errors(format!("./{}", files.install_script_location), |cmd| {
                cmd.current_dir(&files.package_directory)
            });
        handle.done();
        result
    }

    fn re_run_command() {
        let args: Vec<String> = args().collect();
        Executor::with_stdio(args.join(" "), |cmd| cmd);
    }
}
