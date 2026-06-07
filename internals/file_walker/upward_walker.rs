use std::path::PathBuf;

use normalize_path::NormalizePath;

#[derive(Clone)]
pub struct UpwardWalker<'a> {
    pub starting_path: &'a PathBuf,
}

impl UpwardWalker<'_> {
    pub fn new<'a>(starting_path: &'a PathBuf) -> UpwardWalker<'a> {
        UpwardWalker { starting_path }
    }

    pub fn search_for(&self, segment: &str) -> Option<PathBuf> {
        let mut current = self.starting_path.to_owned();
        if let Some(matched_path) = self.match_current_path(&current, segment) {
            return Some(matched_path);
        }
        while let Some(parent) = current.parent()
            && parent.exists()
            && parent.to_string_lossy() != ""
        {
            let parent_buf = parent.to_path_buf();
            if let Some(matched_path) = self.match_current_path(&parent_buf, segment) {
                return Some(matched_path);
            }
            current = parent_buf;
        }
        None
    }

    pub fn find_match(&self, mut matcher: impl FnMut(&PathBuf) -> bool) -> Option<PathBuf> {
        let mut current = self.starting_path.to_owned();
        if matcher(&current) {
            return Some(current);
        }
        while let Some(parent) = current.parent()
            && parent.exists()
            && parent.to_string_lossy() != ""
        {
            let parent_buf = parent.to_path_buf();
            if matcher(&parent_buf) {
                return Some(parent_buf);
            }
            current = parent_buf;
        }
        None
    }

    fn match_current_path(&self, path: &PathBuf, segment: &str) -> Option<PathBuf> {
        let attempting_path = path.join(segment).normalize();
        if attempting_path.exists() {
            return Some(path.to_owned());
        }
        None
    }
}
