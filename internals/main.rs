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
mod configuration;
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
    let versions = RepoKitVersionScope::new(&files);
    let cache = CacheScope::new(&versions.installed_version, &git.commit_hash);
    let node = NodeScope::new(&git.root);
    let bridge = TypeScriptBridge::new(&files);
    let configuration = bridge.parse_configuration();
    let kit = RepoKit::new(RepoKitScope {
        git,
        node,
        files,
        cache,
        bridge,
        versions,
        configuration,
    });
    kit.invoke();
}
