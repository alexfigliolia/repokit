use futures::join;

use crate::{
    context::initializer::Initializer, executor::executor::Executor, logger::logger::Logger,
};

#[derive(Clone)]
pub struct GitScope {
    pub root: String,
    pub commit_hash: String,
}

impl Initializer<(String, String)> for GitScope {
    async fn resolve(&mut self, _: &str) -> (String, String) {
        join!(GitScope::find_root(), GitScope::get_head_commit())
    }
}

impl GitScope {
    pub fn new() -> GitScope {
        let mut instance = GitScope {
            root: "".to_string(),
            commit_hash: "".to_string(),
        };
        let (root, commit) = GitScope::resolve_sync(instance.resolve(""));
        instance.root = root;
        instance.commit_hash = commit;
        instance
    }

    async fn find_root() -> String {
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

    async fn get_head_commit() -> String {
        Executor::exec("git rev-parse HEAD", |cmd| cmd)
    }
}
