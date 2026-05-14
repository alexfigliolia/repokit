use std::path::{Path, PathBuf};

use futures::join;

use crate::{
    context::async_scope::AsyncScope, executor::executor::Executor, logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

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
            GitScope::get_root_commit(),
            GitScope::get_head_commit()
        )
    }
}

impl GitScope {
    async fn find_root() -> Option<PathBuf> {
        if let Some(root) = Executor::exec_with_stdout("git rev-parse --show-toplevel", |cmd| cmd)
            && !root.is_empty()
        {
            let path = Path::new(&root);
            if path.exists() {
                return Some(path.to_path_buf());
            }
        }
        PostProcessor::get().register_task(|| {
            Logger::info(format!(
            "Running {} in your workspace will allow {} to cache file crawls more aggressively and improve performance",
            Logger::with_theme(|theme| theme.highlight("git init")),
            Logger::with_theme(|theme| theme.highlight("Repokit"))
        ).as_str());
        });
        None
    }

    async fn get_head_commit() -> Option<String> {
        Executor::exec_with_stdout("git rev-parse HEAD", |cmd| cmd)
    }

    async fn get_root_commit() -> Option<String> {
        Executor::exec_with_stdout("git rev-list --parents HEAD | tail -1", |cmd| cmd)
    }
}
