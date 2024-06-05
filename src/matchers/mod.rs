use std::fs::DirEntry;

pub mod filenamematcher;

pub enum Traverse {
    Recurse,
    NoRecurse,
}

pub trait Matcher {
    fn process(&self, entry: &DirEntry) -> Traverse;
}
