use std::fs::DirEntry;

use super::Matcher;

pub struct FileNameMatcher {
    file_name: String,
}

impl FileNameMatcher {
    pub fn new(file_name: String) -> Self {
        Self { file_name }
    }
}

impl Matcher for FileNameMatcher {
    fn process(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        if path.is_file() && path.ends_with(&self.file_name) {
            return true;
        }
        false
    }
}
