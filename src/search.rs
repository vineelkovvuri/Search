use std::{fs, path::PathBuf};

use crate::matchers::{Matcher};

pub fn traverse<M>(path: &PathBuf, matcher: &M)
where
    M: Matcher,
{
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let _ = matcher.process(&entry);
            let path = entry.path();
            if path.is_dir() {
                traverse(&entry.path(), matcher);
            }
        }
    }
}
