use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

use ignore::{DirEntry, Error, ParallelVisitor, ParallelVisitorBuilder, WalkState};
use regex::Regex;

use crate::{internal_filesystem::file_builder::FileBuilder, logger::logger::Logger};

pub struct TSFileVisitor {
    root: String,
    paths: Arc<Mutex<Vec<String>>>,
}

impl ParallelVisitor for TSFileVisitor {
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
        let root_replacer = format!("{}/", self.root);
        let repokit_import_matcher =
            Regex::new(r#"(require\(|from[\s*]?)['"]@repokit/core["'][\)]?[;]?$"#).unwrap();
        if let Ok(entry) = entry {
            let path = entry.path();
            let path_string = path.to_str().map_or("", |f| f);
            if entry.file_type().is_some_and(|ft| ft.is_file()) && path_string.ends_with(".ts") {
                let mut open_comment = false;
                let file: File = FileBuilder::open(path_string, |_| Logger::open_file_error());
                let reader: BufReader<File> = BufReader::new(file);
                for line_result in reader.lines() {
                    let unwrapped = line_result.unwrap();
                    let line = unwrapped.trim();
                    if !open_comment && line.starts_with("/**") {
                        open_comment = true;
                        continue;
                    }
                    if open_comment {
                        if line == "*/" {
                            open_comment = false;
                        }
                        continue;
                    }
                    if repokit_import_matcher.is_match(line) {
                        let mut vector = self.paths.lock().unwrap();
                        vector.push(path_string.to_string().replace(&root_replacer, ""));
                        break;
                    }
                    if !line.is_empty()
                        && !line.starts_with("import ")
                        && !line.contains("require(")
                        && !(line.starts_with("//") || line.starts_with("/*"))
                    {
                        break;
                    }
                }
            }
        }
        WalkState::Continue
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
            paths: self.paths.clone(),
            root: self.root.to_string(),
        })
    }
}
