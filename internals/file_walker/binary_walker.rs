use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use ignore::{DirEntry, Error, ParallelVisitor, ParallelVisitorBuilder, WalkState};

pub struct BinaryVisitor {
    result: Arc<Mutex<Option<PathBuf>>>,
}

impl ParallelVisitor for BinaryVisitor {
    fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
        let result = self.result.lock().unwrap();
        if result.is_some() {
            return WalkState::Quit;
        }
        drop(result);
        if let Ok(file_entry) = entry
            && file_entry.file_type().is_some_and(|ft| ft.is_dir())
            && file_entry
                .file_name()
                .to_string_lossy()
                .starts_with("native-binary")
        {
            let binary_names = ["repokit", "repokit.exe"];
            for binary in binary_names {
                let binary_path = file_entry.path().join(binary);
                if binary_path.exists() {
                    *self.result.lock().unwrap() = Some(binary_path.to_path_buf());
                    return WalkState::Quit;
                }
            }
        }
        WalkState::Continue
    }
}

pub struct BinaryVisitorBuilder<'a> {
    pub result: &'a Arc<Mutex<Option<PathBuf>>>,
}

impl<'a> BinaryVisitorBuilder<'a> {
    pub fn new(result: &'a Arc<Mutex<Option<PathBuf>>>) -> BinaryVisitorBuilder<'a> {
        BinaryVisitorBuilder { result }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for BinaryVisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(BinaryVisitor {
            result: self.result.clone(),
        })
    }
}
