use std::process::exit;

use futures::join;

use crate::{
    context::initializer::Initializer, executor::executor::Executor, logger::logger::Logger,
    post_processing::post_processor::PostProcessor,
};

#[derive(Clone)]
pub struct GitScope {
    pub root: String,
    pub root_commit_hash: Option<String>,
    pub head_commit_hash: Option<String>,
}

impl Initializer<(), &str> for GitScope {
    async fn resolve(&mut self, _: &str) {
        let (root, root_commit, head_commit) = join!(
            GitScope::find_root(),
            GitScope::get_root_commit(),
            GitScope::get_head_commit()
        );
        self.root = root;
        self.root_commit_hash = root_commit;
        self.head_commit_hash = head_commit;
    }
}

impl GitScope {
    pub fn new() -> GitScope {
        let mut instance = GitScope {
            root: "".to_string(),
            root_commit_hash: None,
            head_commit_hash: None,
        };
        GitScope::resolve_sync(instance.resolve(""));
        instance
    }

    async fn find_root() -> String {
        if let Some(root) = Executor::exec_with_stdout("git rev-parse --show-toplevel", |cmd| cmd)
            && !root.is_empty()
        {
            return root;
        }
        Logger::exit_with_info(
            format!(
                "To start using {}, please initialize your git repository by running {}",
                Logger::with_theme(|theme| theme.highlight("Repokit")),
                Logger::with_theme(|theme| theme.highlight("git init"))
            )
            .as_str(),
        );
        PostProcessor::get().flush();
        exit(0);
    }

    async fn get_head_commit() -> Option<String> {
        Executor::exec_with_stdout("git rev-parse HEAD", |cmd| cmd)
    }

    async fn get_root_commit() -> Option<String> {
        Executor::exec_with_stdout("git rev-list --parents HEAD | tail -1", |cmd| cmd)
    }
}
