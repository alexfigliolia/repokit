use std::{
    env::args,
    io::{Error, ErrorKind},
};

use terminal_spinners::{BOUNCING_BALL, SpinnerBuilder};

use crate::{context::file_system::FileSystem, executor::executor::Executor};

pub struct RepoKitVersionResolver;

impl RepoKitVersionResolver {
    pub fn hop_to_installed_version(files: &FileSystem) -> Result<(), Error> {
        if files.install_script_path.is_absolute() && files.install_script_path.exists() {
            if let Some(errors) = RepoKitVersionResolver::run_post_install(files) {
                println!("{errors}");
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "post install script failed to execute",
                ));
            } else {
                RepoKitVersionResolver::re_run_command();
                return Ok(());
            }
        }
        Err(Error::new(
            ErrorKind::NotFound,
            "post install script not found",
        ))
        // TODO attempt recovery via re-install from npm
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
