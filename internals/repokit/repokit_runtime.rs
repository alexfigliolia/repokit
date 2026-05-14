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
        let git_init = runtime.spawn(GitScope::new());
        let install_init = runtime.spawn(InstallationScope::new());
        let git = block_on(git_init).unwrap();
        let installation = block_on(install_init).unwrap();
        let files = FileSystem::new(&installation.install_path);
        let mut node = NodeScope::new(&installation.install_path);
        let caches = block_on(CacheScope::new(&git, &runtime));
        let configuration = TypeScriptBridge::parse_configuration(&files, &mut node);
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
