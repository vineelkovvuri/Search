use std::fs::DirEntry;

use super::{Matcher, Traverse};

pub struct FileSizeMatcher {
    file_size_min: u64,
    file_size_max: u64,
}

impl FileSizeMatcher {
    pub fn new(file_size_min: u64, file_size_max: u64) -> Self {
        Self {
            file_size_min,
            file_size_max,
        }
    }
}

impl Matcher for FileSizeMatcher {
    fn process(&self, entry: &DirEntry) -> Traverse {
        let path = entry.path();

        if path.is_dir() {
            Traverse::Recurse
        } else {
            if let Ok(metadata) = path.metadata() {
                if metadata.len() <= self.file_size_max && metadata.len() >= self.file_size_min {
                    println!("{:?}", path.to_str().unwrap());
                }
            }
            Traverse::NoRecurse
        }
    }
}
