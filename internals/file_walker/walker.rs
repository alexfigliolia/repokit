use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::{Arc, LazyLock, Mutex},
};

use ignore::{DirEntry, Error, ParallelVisitor, ParallelVisitorBuilder, WalkState};
use regex::Regex;
use tokio::task::JoinSet;

use crate::{internal_filesystem::file_builder::FileBuilder, logger::logger::Logger};

pub struct TSFileVisitor {
    root: String,
    paths: Arc<Mutex<Vec<String>>>,
}

static REPOKIT_IMPORT_MATCHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(require\(|from[\s*]?)['"]@repokit/native-test["'][\)]?[;]?$"#).unwrap()
});

impl ParallelVisitor for TSFileVisitor {
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
        let root_replacer = format!("{}/", self.root);
        if let Ok(file_entry) = entry
            && file_entry.file_type().is_some_and(|ft| ft.is_file())
            && file_entry.file_name().to_string_lossy().ends_with(".ts")
            && let Some(matched_path) = TSFileVisitor::on_file(file_entry.path())
        {
            let mut vector = self.paths.lock().unwrap();
            vector.push(matched_path.to_string_lossy().replace(&root_replacer, ""));
        }

        WalkState::Continue
    }
}

impl TSFileVisitor {
    #[tokio::main]
    pub async fn traverse_list(root: &str, path_list: Vec<String>) -> Vec<String> {
        let mut handles = JoinSet::new();
        let root_replacer = format!("{}/", root);
        for path in path_list {
            handles.spawn(async move {
                if let Some(result) = TSFileVisitor::on_file(Path::new(&path)) {
                    return Some(result.to_owned());
                }
                None
            });
        }
        handles
            .join_all()
            .await
            .iter()
            .filter_map(|result| {
                result
                    .as_ref()
                    .map(|path| path.to_string_lossy().replace(&root_replacer, ""))
            })
            .collect()
    }

    pub fn on_file(path: &Path) -> Option<&Path> {
        let mut open_comment = false;
        let file: File = FileBuilder::open(path, |_| Logger::open_file_error());
        let reader: BufReader<File> = BufReader::new(file);
        for line_result in reader.lines() {
            let unwrapped = line_result.unwrap();
            let line = unwrapped.trim();
            if !open_comment && line.starts_with("/*") {
                open_comment = true;
                continue;
            }
            if open_comment {
                if line.ends_with("*/") {
                    open_comment = false;
                }
                continue;
            }
            if REPOKIT_IMPORT_MATCHER.is_match(line) {
                return Some(path);
            }
            if !line.is_empty()
                && !line.starts_with("import ")
                && !line.contains("require(")
                && !(line.starts_with("//") || line.starts_with("/*"))
            {
                break;
            }
        }
        None
    }
}

pub struct TSFileVisitorBuilder<'a> {
    pub root: &'a str,
    pub paths: &'a Arc<Mutex<Vec<String>>>,
}

impl<'a> TSFileVisitorBuilder<'a> {
    pub fn new(root: &'a str, paths: &'a Arc<Mutex<Vec<String>>>) -> TSFileVisitorBuilder<'a> {
        TSFileVisitorBuilder { paths, root }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for TSFileVisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(TSFileVisitor {
            root: self.root.to_string(),
            paths: self.paths.clone(),
        })
    }
}
