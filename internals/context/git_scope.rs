use std::path::{Path, PathBuf};

use futures::join;

use crate::{context::async_scope::AsyncScope, executor::executor::Executor};

#[derive(Clone)]
pub struct GitScope {
    pub root_path: Option<PathBuf>,
    pub root_commit_hash: Option<String>,
    pub head_commit_hash: Option<String>,
}

impl AsyncScope<(Option<PathBuf>, Option<String>, Option<String>)> for GitScope {
    async fn new() -> Self {
        let (root_path, root_commit_hash, head_commit_hash) = GitScope::resolve().await;
        Self {
            root_path,
            root_commit_hash,
            head_commit_hash,
        }
    }

    async fn resolve() -> (Option<PathBuf>, Option<String>, Option<String>) {
        join!(
            GitScope::find_root(),
            GitScope::exec_with_non_empty_result("git rev-list --parents HEAD | tail -1"),
            GitScope::exec_with_non_empty_result("git rev-parse HEAD")
        )
    }
}

impl GitScope {
    async fn find_root() -> Option<PathBuf> {
        if let Some(root) =
            GitScope::exec_with_non_empty_result("git rev-parse --show-toplevel").await
        {
            let path = Path::new(&root);
            if path.exists() {
                return Some(path.to_path_buf());
            }
        }
        None
    }

    async fn exec_with_non_empty_result(command: &str) -> Option<String> {
        if let Some(result) = Executor::exec_with_stdout(command, |cmd| cmd)
            && !result.is_empty()
        {
            return Some(result);
        }
        None
    }
}
