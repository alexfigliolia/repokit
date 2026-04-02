use crate::{
    configuration::typescript_command::TypescriptCommand,
    executables::internal_executable_definition::RepoKitScope,
    git_scope::git_scope::GitScope,
    repokit::{repokit::RepoKit, runtime_compiler::RuntimeCompiler},
};

mod argv;
mod configuration;
mod executables;
mod executor;
mod file_walker;
mod git_scope;
mod internal_commands;
mod internal_filesystem;
mod logger;
mod post_processing;
mod repokit;
mod themes;
mod validations;

fn main() {
    let scope = GitScope::new();
    RuntimeCompiler::hop_to_runtime_version(&scope.root);
    let config = TypescriptCommand::new(&scope.root).parse_configuration();
    let kit = RepoKit::new(RepoKitScope::new(scope, config));
    kit.invoke();
}
