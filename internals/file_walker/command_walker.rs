use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex},
};

use ignore::{DirEntry, Error, ParallelVisitor, ParallelVisitorBuilder, WalkState};
use regex::Regex;
use tokio::task::JoinSet;

use crate::{internal_filesystem::file_builder::FileBuilder, logger::logger::Logger};

pub struct TSCommandVisitor {
    root: PathBuf,
    paths: Arc<Mutex<Vec<String>>>,
    root_replacer: String,
}

static REPOKIT_IMPORT_MATCHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(require\(|from[\s*]?)['"]@repokit/core["'][\)]?[;]?$"#).unwrap()
});

impl ParallelVisitor for TSCommandVisitor {
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
        let root_replacer = format!("{}/", self.root_replacer);
        if let Ok(file_entry) = entry
            && file_entry.file_type().is_some_and(|ft| ft.is_file())
            && file_entry.file_name().to_string_lossy().ends_with(".ts")
            && let Some(matched_path) =
                TSCommandVisitor::on_file(&self.root.join(file_entry.path()))
        {
            let mut vector = self.paths.lock().unwrap();
            vector.push(matched_path.to_string_lossy().replace(&root_replacer, ""));
        }

        WalkState::Continue
    }
}

impl TSCommandVisitor {
    #[tokio::main]
    pub async fn traverse_list(root: &PathBuf, path_list: Vec<String>) -> Vec<String> {
        let mut handles = JoinSet::new();
        let root_replacer = format!("{}/", root.to_string_lossy());
        for path in path_list {
            let root_clone = root.to_owned();
            handles.spawn(async move {
                if let Some(result) = TSCommandVisitor::on_file(&root_clone.join(Path::new(&path)))
                {
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

pub struct TSCommandVisitorBuilder<'a> {
    pub root: &'a PathBuf,
    pub paths: &'a Arc<Mutex<Vec<String>>>,
}

impl<'a> TSCommandVisitorBuilder<'a> {
    pub fn new(
        root: &'a PathBuf,
        paths: &'a Arc<Mutex<Vec<String>>>,
    ) -> TSCommandVisitorBuilder<'a> {
        TSCommandVisitorBuilder { paths, root }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for TSCommandVisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(TSCommandVisitor {
            paths: self.paths.clone(),
            root: self.root.to_owned(),
            root_replacer: self.root.to_string_lossy().to_string(),
        })
    }
}
