use std::fs::DirEntry;

use super::{Matcher, Traverse};

pub struct FileSizeMatcher {
    size: (u64, u64),
}

impl FileSizeMatcher {
    pub fn new(size: (u64, u64)) -> Self {
        Self {
            size
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
                if metadata.len() <= self.size.1 && metadata.len() >= self.size.0 {
                    println!("{:?}", path.to_str().unwrap());
                }
            }
            Traverse::NoRecurse
        }
    }
}
