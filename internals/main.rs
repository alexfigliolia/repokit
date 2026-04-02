use crate::{
    configuration::typescript_command::TypescriptCommand,
    executables::internal_executable_definition::RepoKitScope,
    initializers::{git_scope::GitScope, repokit_version_scope::RepoKitVersionScope},
    repokit::repokit::RepoKit,
};

mod argv;
mod configuration;
mod executables;
mod executor;
mod file_walker;
mod initializers;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod post_processing;
mod repokit;
mod themes;
mod validations;

fn main() {
    let git = GitScope::new();
    let versions = RepoKitVersionScope::new(&git.root);
    let configuration = TypescriptCommand::new(&git.root).parse_configuration();
    let kit = RepoKit::new(RepoKitScope {
        git,
        versions,
        configuration,
    });
    kit.invoke();
}
