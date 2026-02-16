use crate::{
    internal_commands::typescript_command::TypescriptCommand,
    internal_filesystem::internal_filesystem::InternalFileSystem, repokit::repokit::RepoKit,
};

mod configuration;
mod executables;
mod executor;
mod file_walker;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod repokit;
mod validations;

fn main() {
    let root = InternalFileSystem::find_root();
    let config = TypescriptCommand::new(&root).parse_configuration();
    let kit = RepoKit::new(root, config);
    kit.invoke();
}
