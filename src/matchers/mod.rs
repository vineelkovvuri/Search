use std::fs::DirEntry;

pub mod filedatematcher;
pub mod filenamematcher;
pub mod filesizematcher;

// pub enum Traverse {
//     Recurse,
//     NoRecurse,
// }

pub trait Matcher {
    fn process(&self, entry: &DirEntry) -> bool;
}

pub struct MatcherPipeline
{
    matchers: Vec<Box<dyn Matcher>>,
}

impl MatcherPipeline
{
    pub fn new(matchers: Vec<Box<dyn Matcher>>) -> Self {
        Self { matchers }
    }
}

impl Matcher for MatcherPipeline
{
    fn process(&self, entry: &DirEntry) -> bool {
        for matcher in &self.matchers {
            if !matcher.process(entry) {
                return false;
            }
        }

        let path = entry.path();
        println!("{}", path.to_str().unwrap());
        true
    }
}
