use std::sync::{Arc, Mutex, MutexGuard};

use ignore::WalkBuilder;

use crate::{file_walker::walker::TSFileVisitorBuilder, repokit::repokit_runtime::RepoKitRuntime};

pub struct FileWalker {
    pub command_paths: Arc<Mutex<Vec<String>>>,
}

impl FileWalker {
    pub fn new() -> Self {
        let instance = FileWalker {
            command_paths: Arc::new(Mutex::new(Vec::new())),
        };
        instance.resolve();
        instance
    }

    pub fn get(&self) -> MutexGuard<'_, Vec<String>> {
        self.command_paths.lock().unwrap()
    }

    fn resolve(&self) {
        if RepoKitRuntime::with_runtime(|runtime| runtime.caches.crawl_cache.crawl_cache_enabled())
        {
            self.from_cache();
        } else {
            self.crawl_file_system();
        }
    }

    fn crawl_file_system(&self) {
        RepoKitRuntime::with_runtime(|runtime| {
            let mut visitor = TSFileVisitorBuilder::new(&runtime.git.root, &self.command_paths);
            WalkBuilder::new(&runtime.files.git_root_path)
                .build_parallel()
                .visit(&mut visitor);
            if let Some(head_commit) = &runtime.git.head_commit_hash {
                let files = self.command_paths.lock().unwrap();
                runtime
                    .caches
                    .crawl_cache
                    .cache_crawl_results([head_commit.to_owned(), files.join("\n")].join("\n"));
            }
        });
    }

    fn from_cache(&self) {
        let mut paths_to_search = self.command_paths.lock().unwrap();
        RepoKitRuntime::with_runtime(|runtime| {
            if let Some(files_to_crawl) = &runtime.caches.crawl_cache.files_to_crawl {
                for file in files_to_crawl {
                    paths_to_search.push(file.to_owned())
                }
            }
        })
    }
}
