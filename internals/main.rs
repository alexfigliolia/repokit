use crate::{post_processing::post_processor::PostProcessor, repokit::repokit::RepoKit};

mod argv;
mod context;
mod executables;
mod executor;
mod file_walker;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod post_processing;
mod repokit;
mod themes;
mod validations;

fn main() {
    RepoKit::new().invoke();
    PostProcessor::get().flush();
}
