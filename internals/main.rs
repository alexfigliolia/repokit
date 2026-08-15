use std::panic;

use crate::{post_processing::post_processor::PostProcessor, repokit::repokit::RepoKit};

mod argv;
mod caches;
mod context;
mod executables;
mod executor;
mod file_walker;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod post_processing;
mod prompts;
mod repokit;
mod themes;
mod typescript_library;
mod validations;

fn main() {
    panic::set_hook(Box::new(|_| {}));
    let _ = panic::catch_unwind(|| {
        RepoKit::new().invoke();
    });
    PostProcessor::get().flush();
}
