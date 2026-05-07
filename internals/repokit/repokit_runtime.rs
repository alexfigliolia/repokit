use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::{
    context::{
        cache_scope::CacheScope, file_system::FileSystem, git_scope::GitScope,
        node_scope::NodeScope, typescript_bridge::TypeScriptBridge,
    },
    repokit::repokit_config::RepoKitConfig,
};

pub struct RepoKitRuntime {
    pub git: GitScope,
    pub node: NodeScope,
    pub files: FileSystem,
    pub caches: CacheScope,
    pub configuration: RepoKitConfig,
}

static REPOKIT_RUNTIME: LazyLock<Mutex<RepoKitRuntime>> =
    LazyLock::new(|| Mutex::new(RepoKitRuntime::new()));

impl RepoKitRuntime {
    pub fn new() -> RepoKitRuntime {
        let git = GitScope::new();
        let files = FileSystem::new(&git.root);
        let caches = CacheScope::new(&git);
        let mut node = NodeScope::new(&git.root);
        let configuration = TypeScriptBridge::parse_configuration(&files, &mut node);
        RepoKitRuntime {
            git,
            node,
            files,
            caches,
            configuration,
        }
    }

    pub fn with_runtime<R>(mut func: impl FnMut(MutexGuard<'_, RepoKitRuntime>) -> R) -> R {
        let registry = REPOKIT_RUNTIME.lock().unwrap();
        func(registry)
    }
}
