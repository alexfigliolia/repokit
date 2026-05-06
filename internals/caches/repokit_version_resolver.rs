use std::env::args;

use crate::{
    context::file_system::FileSystem, executor::executor::Executor, logger::logger::Logger,
};

pub struct RepoKitVersionResolver;

impl RepoKitVersionResolver {
    pub fn hop_to_installed_version(files: &FileSystem) {
        if files.install_script_path.is_absolute() && files.install_script_path.exists() {
            if let Some(errors) = RepoKitVersionResolver::run_post_install(files) {
                println!("{errors}");
                Logger::error("My installer failed");
                RepoKitVersionResolver::on_error();
            } else {
                RepoKitVersionResolver::re_run_command();
            }
        } else {
            Logger::error("I couldn't find my installer");
            RepoKitVersionResolver::on_error();
        }
        panic!();
    }

    fn run_post_install(files: &FileSystem) -> Option<String> {
        // let handle = SpinnerBuilder::new()
        //     .spinner(&BOUNCING_BALL)
        //     .text(" Installing")
        //     .start();
        
        // handle.done();
        Executor::exec_with_errors(format!("./{}", files.install_script_location), |cmd| {
                cmd.current_dir(&files.package_directory)
            })
    }

    fn re_run_command() {
        let args: Vec<String> = args().collect();
        Executor::with_stdio(args.join(" "), |cmd| cmd);
    }

    fn on_error() {
        Logger::info("Please file a bug here");
        Logger::log_issue_link();
    }
}
