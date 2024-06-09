use std::fs::DirEntry;

use super::Matcher;

pub struct FileSizeMatcher {
    size: (u64, u64),
}

impl FileSizeMatcher {
    pub fn new(size: (u64, u64)) -> Self {
        Self { size }
    }
}

impl Matcher for FileSizeMatcher {
    fn process(&self, entry: &DirEntry) -> bool {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = path.metadata() {
                if metadata.len() <= self.size.1 && metadata.len() >= self.size.0 {
                    // println!("{:?}", path.to_str().unwrap());
                    return true;
                }
            }
        }

        false
    }
}
