use crate::{
    context::{
        cache_scope::CacheScope, file_system::FileSystem, git_scope::GitScope,
        node_scope::NodeScope, repokit_version_scope::RepoKitVersionScope,
        typescript_bridge::TypeScriptBridge,
    },
    executables::internal_executable_definition::RepoKitScope,
    repokit::repokit::RepoKit,
};

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
    let git = GitScope::new();
    let files = FileSystem::new(&git.root);
    let cache = CacheScope::new(&git);
    let versions = RepoKitVersionScope::new((&files, &cache));
    let mut node = NodeScope::new(&git.root);
    let configuration = TypeScriptBridge::parse_configuration(&files, &mut node);
    let kit = RepoKit::new(RepoKitScope {
        git,
        node,
        files,
        cache,
        versions,
        configuration,
    });
    kit.invoke();
}
