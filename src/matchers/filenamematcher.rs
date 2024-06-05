use std::fs::DirEntry;

use super::{Matcher, Traverse};

pub struct FileNameMatcher {
    file_name: String,
}

impl FileNameMatcher {
    pub fn new(file_name: String) -> Self {
        Self { file_name }
    }
}

impl Matcher for FileNameMatcher {
    fn process(&self, entry: &DirEntry) -> Traverse {
        let path = entry.path();

        if path.is_dir() {
            Traverse::Recurse
        } else {
            if path.ends_with(&self.file_name) {
                println!("{:?}", path.to_str().unwrap());
            }
            Traverse::NoRecurse
        }
    }
}
