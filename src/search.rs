use std::{fs, path::PathBuf};

use crate::matchers::{Matcher, Traverse};

pub fn traverse<M>(path: &PathBuf, matcher: &M)
where
    M: Matcher,
{
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            match matcher.process(&entry) {
                Traverse::Recurse => {
                    traverse(&entry.path(), matcher);
                }
                Traverse::NoRecurse => {}
            }
        }
    }
}
