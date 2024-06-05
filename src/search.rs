use std::{fs, path::PathBuf};

use crate::matchers::{Matcher, Traverse};

pub fn search2<M>(path: &PathBuf, matcher: &M)
where
    M: Matcher,
{
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            match matcher.process(&entry) {
                Traverse::Recurse => {
                    search2(&entry.path(), matcher);
                }
                Traverse::NoRecurse => {}
            }
        }
    }
}
