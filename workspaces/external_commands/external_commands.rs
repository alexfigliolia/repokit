use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use walkdir::{DirEntry, Error, WalkDir};

use crate::{
    concurrency::thread_pool::ThreadPool, devkit::interfaces::DevKitCommand,
    executables::intenal_executable::InternalExecutable,
    internal_commands::typescript_command::TypescriptCommand, logger::logger::Logger,
};

pub struct ExternalCommands {
    pub root: String,
}

impl ExternalCommands {
    pub fn new(root: String) -> ExternalCommands {
        ExternalCommands { root }
    }

    pub async fn find_all(&self) -> HashMap<String, DevKitCommand> {
        let mut paths: Vec<String> = vec![];
        let mut pool = ThreadPool::new(None, None);
        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter(|e| self.allowed(e))
            .map(|e| e.ok())
        {
            let unwrapped = entry.expect("path");
            let path = unwrapped.path().to_owned();
            if path.is_file() && path.extension().map(|ext| ext == "ts").unwrap_or(false) {
                let clone = path.clone();
                let async_task = pool.spawn(move || ExternalCommands::read(&path));
                if async_task.await.unwrap() {
                    paths.push((clone).into_os_string().into_string().expect("stringify"));
                }
            }
        }
        self.collect_instances(paths)
    }

    pub fn validate(
        internals: &HashMap<String, Box<dyn InternalExecutable>>,
        externals: &HashMap<String, DevKitCommand>,
    ) {
        for (name, command) in externals {
            if internals.contains_key(name) {
                Logger::info(
                    format!(
                        "I encountered a command named {} that conflicts with one of my internals",
                        Logger::cyan_bright(name),
                    )
                    .as_str(),
                );
                Logger::info(
                    format!(
                        "{}{}",
                        Logger::indent(None),
                        Logger::cyan_bright(&command.location),
                    )
                    .as_str(),
                );
                Logger::exitWithInfo("Please rename it");
            }
        }
    }

    fn collect_instances(&self, paths: Vec<String>) -> HashMap<String, DevKitCommand> {
        let mut map = HashMap::new();
        let commands = TypescriptCommand::parse_commands(paths);
        for command in commands {
            map.insert(command.name.clone(), command);
        }
        map
    }

    fn read(path: &Path) -> bool {
        let file: File = File::open(path).expect("file");
        let reader: BufReader<File> = BufReader::new(file);
        for line_result in reader.lines() {
            let line: String = line_result.expect("line");
            if line.contains("new DevKitCommand(") {
                return true;
            }
        }
        false
    }

    fn allowed(&self, entry: &Result<DirEntry, Error>) -> bool {
        entry.is_ok()
            && !self.black_list_dirs(
                entry
                    .as_ref()
                    .expect("path")
                    .path()
                    .to_str()
                    .expect("stringify"),
            )
    }

    fn black_list_dirs(&self, path: &str) -> bool {
        let restricted_paths = [".", "node_modules", "target"];
        let restricted_extensions = [".lock", "internal_commands/command_template.ts"];
        let relative_path = path.replace(format!("{}/", &self.root).as_str(), "");
        if ExternalCommands::restrict(
            &relative_path,
            &restricted_paths,
            RestrictDirection::Forwards,
        ) {
            return true;
        }
        if ExternalCommands::restrict(
            &relative_path,
            &restricted_extensions,
            RestrictDirection::Backwards,
        ) {
            return true;
        }
        let components = relative_path.split('/');
        for token in components {
            if token.starts_with(".") {
                return true;
            }
            for restricted_path in restricted_paths {
                if token == restricted_path {
                    return true;
                }
            }
        }
        false
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
