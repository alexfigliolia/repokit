use crate::{
    executor::executor::Executor, internal_commands::typescript_command::TypescriptCommand,
    repokit::repokit::RepoKit,
};

mod concurrency;
mod configuration;
mod executables;
mod executor;
mod external_commands;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod repokit;
mod validations;

fn main() {
    let root = Executor::exec("git rev-parse --show-toplevel", |cmd| cmd);
    let config = TypescriptCommand::new(root.clone()).parse_configuration();
    let kit = RepoKit::new(root, config);
    kit.invoke();
}
