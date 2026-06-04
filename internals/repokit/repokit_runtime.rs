use std::sync::{LazyLock, Mutex, MutexGuard};

use futures::executor::block_on;
use tokio::runtime::Builder;

use crate::{
    context::{
        async_scope::AsyncScope, cache_scope::CacheScope, file_system::FileSystem,
        git_scope::GitScope, installation_scope::InstallationScope, node_scope::NodeScope,
        typescript_bridge::TypeScriptBridge,
    },
    repokit::repokit_config::RepoKitConfig,
};

pub struct RepoKitRuntime {
    pub git: GitScope,
    pub node: NodeScope,
    pub files: FileSystem,
    pub caches: CacheScope,
    pub configuration: RepoKitConfig,
    pub installation: InstallationScope,
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
        let caches = block_on(CacheScope::new(&git, &runtime));
        let p1 = installation.install_path.to_path_buf();
        let p2 = installation.install_path.to_path_buf();
        let files_init = runtime.spawn(async move { FileSystem::new(&p1) });
        let node_init = runtime.spawn(async move { NodeScope::new(&p2) });
        let files = block_on(files_init).unwrap();
        let mut node = block_on(node_init).unwrap();
        let configuration = TypeScriptBridge::parse_configuration(&files, &mut node);
        runtime.shutdown_background();
        RepoKitRuntime {
            git,
            node,
            files,
            caches,
            configuration,
            installation,
        }
    }

    pub fn with_runtime<R>(mut func: impl FnMut(MutexGuard<'_, RepoKitRuntime>) -> R) -> R {
        let registry = REPOKIT_RUNTIME.lock().unwrap();
        func(registry)
    }
}
