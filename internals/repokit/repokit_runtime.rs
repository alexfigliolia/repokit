use std::sync::{LazyLock, Mutex, MutexGuard};

use futures::executor::block_on;
use tokio::runtime::Builder;

use crate::{
    context::{
        async_scope::AsyncScope, cache_scope::CacheScope, git_scope::GitScope,
        installation_scope::InstallationScope, node_scope::NodeScope,
        typescript_library_installation::TypeScriptLibraryInstallation,
    },
    repokit::repokit_config::RepoKitConfig,
    typescript_library::typescript_bridge::TypeScriptBridge,
};

pub struct RepoKitRuntime {
    pub git: GitScope,
    pub node: NodeScope,
    pub caches: CacheScope,
    pub configuration: RepoKitConfig,
    pub typescript_library: TypeScriptLibraryInstallation,
}

static REPOKIT_RUNTIME: LazyLock<Mutex<RepoKitRuntime>> =
    LazyLock::new(|| Mutex::new(RepoKitRuntime::new()));

impl RepoKitRuntime {
    pub fn new() -> RepoKitRuntime {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let git_init = GitScope::new();
        let installation_init = runtime.spawn(async move { InstallationScope::new() });
        let git = block_on(git_init);
        let installation = block_on(installation_init).unwrap();
        let cache_init = CacheScope::new(&git, &runtime);
        let p1 = installation.install_path.to_path_buf();
        let p2 = installation.install_path.to_path_buf();
        let library_init = runtime.spawn(async move { TypeScriptLibraryInstallation::new(&p1) });
        let node_init = runtime.spawn(async move { NodeScope::new(&p2) });
        let caches = block_on(cache_init);
        let typescript_library = block_on(library_init).unwrap();
        let mut node = block_on(node_init).unwrap();
        let configuration = TypeScriptBridge::parse_configuration(&typescript_library, &mut node);
        runtime.shutdown_background();
        RepoKitRuntime {
            git,
            node,
            caches,
            configuration,
            typescript_library,
        }
    }

    pub fn with_runtime<R>(mut func: impl FnMut(MutexGuard<'_, RepoKitRuntime>) -> R) -> R {
        let registry = REPOKIT_RUNTIME.lock().unwrap();
        func(registry)
    }
}
