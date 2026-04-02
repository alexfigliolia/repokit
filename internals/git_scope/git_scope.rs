use futures::executor::block_on;
use tokio_thread_pool::ThreadPool;

use crate::{executor::executor::Executor, logger::logger::Logger};

pub struct GitScope {
    pub root: String,
    pub commit_hash: String,
}

impl GitScope {
    pub fn new() -> GitScope {
        let mut instance = GitScope {
            root: "".to_string(),
            commit_hash: "".to_string(),
        };
        block_on(instance.resolve());
        instance
    }

    async fn resolve(&mut self) -> &mut GitScope {
        let mut pool = ThreadPool::new(None, None, None);
        let root_handle = pool.spawn(GitScope::find_root);
        let commit_handle = pool.spawn(|| Executor::exec("git rev-parse HEAD", |cmd| cmd));
        if let Ok(root) = root_handle.await
            && let Ok(commit) = commit_handle.await
        {
            self.root = root;
            self.commit_hash = commit;
        }
        self
    }

    fn find_root() -> String {
        let root = Executor::exec("echo $(git rev-parse --show-toplevel 2>/dev/null)", |cmd| {
            cmd
        });
        if root.is_empty() {
            Logger::exit_with_info(
                format!(
                    "To start using {}, please initialize your git repository by running {}",
                    Logger::with_theme(|theme| theme.highlight("Repokit")),
                    Logger::with_theme(|theme| theme.highlight("git init"))
                )
                .as_str(),
            );
        }
        root
    }
}
