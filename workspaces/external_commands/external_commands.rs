use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use jwalk::WalkDir;

use crate::{
    concurrency::thread_pool::ThreadPool, internal_commands::typescript_command::TypescriptCommand,
    repokit::interfaces::RepoKitCommand,
};

pub struct ExternalCommands {
    pub root: String,
}

impl ExternalCommands {
    pub fn new(root: String) -> ExternalCommands {
        ExternalCommands { root }
    }

    pub async fn find_all(&self) -> Vec<RepoKitCommand> {
        let mut paths: Vec<String> = vec![];
        let mut pool = ThreadPool::new(None, None);
        for entry in WalkDir::new(&self.root).into_iter().filter_map(|e| {
            if e.is_err() {
                return None;
            }
            let option = e.ok();
            match option {
                Some(file) => {
                    let path = file.path();
                    if file.file_type().is_file()
                        && path.extension().is_some_and(|ext| ext == "ts")
                        && self.allowed(path.to_str().expect("exists"))
                    {
                        return Some(file);
                    }
                    None
                }
                None => None,
            }
        }) {
            let path = entry.path();
            let clone = path.clone();
            let async_task = pool.spawn(move || ExternalCommands::read(&path));
            if async_task.await.unwrap() {
                paths.push(
                    (clone)
                        .into_os_string()
                        .into_string()
                        .expect("stringify")
                        .replace(&self.root, ""),
                );
            }
        }
        pool.pool.shutdown_background();
        TypescriptCommand::new(self.root.clone()).parse_commands(paths)
    }

    fn read(path: &Path) -> bool {
        let file: File = File::open(path).expect("file");
        let reader: BufReader<File> = BufReader::new(file);
        for line_result in reader.lines() {
            let line: String = line_result.expect("line");
            if line.contains("new RepoKitCommand(") {
                return true;
            }
        }
        false
    }

    fn allowed(&self, path: &str) -> bool {
        let restricted_paths = ["node_modules", "target", "dist"];
        let restricted_extensions = ["templates/command_template.ts"];
        let relative_path = path.replace(format!("{}/", &self.root).as_str(), "");
        if ExternalCommands::restrict(
            &relative_path,
            &restricted_paths,
            RestrictDirection::Forwards,
        ) {
            return false;
        }
        if ExternalCommands::restrict(
            &relative_path,
            &restricted_extensions,
            RestrictDirection::Backwards,
        ) {
            return false;
        }
        let components = relative_path.split('/');
        for token in components {
            for restricted_path in restricted_paths {
                if token == restricted_path {
                    return false;
                }
            }
        }
        true
    }

    fn restrict(path: &str, tokens: &[&str], direction: RestrictDirection) -> bool {
        for restricted in tokens {
            match direction {
                RestrictDirection::Forwards => {
                    if path.starts_with(restricted) {
                        return true;
                    }
                }
                RestrictDirection::Backwards => {
                    if path.ends_with(restricted) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

enum RestrictDirection {
    Forwards,
    Backwards,
}
