use crate::{
    configuration::typescript_command::TypescriptCommand,
    internal_filesystem::internal_filesystem::InternalFileSystem,
    repokit::{repokit::RepoKit, runtime_compiler::RuntimeCompiler},
};

mod argv;
mod configuration;
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
    let root = InternalFileSystem::find_root();
    RuntimeCompiler::hop_to_runtime_version(&root);
    let config = TypescriptCommand::new(&root).parse_configuration();
    let kit = RepoKit::new(&root, config);
    kit.invoke();
}
