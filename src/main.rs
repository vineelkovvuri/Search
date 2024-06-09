use std::path::Path;

use cmd::parse_command_line;
use matchers::{
    filedatematcher::{FileDate, FileDateMatcher},
    filenamematcher::FileNameMatcher,
    filesizematcher::FileSizeMatcher,
    Matcher, MatcherPipeline,
};
use search::traverse;

mod cmd;
mod matchers;
mod search;

struct SearchOptions {
    name: Option<String>,
    path: Option<String>,
    debug: Option<bool>,
    size: Option<(u64, u64)>,
    date: Option<(FileDate, FileDate)>,
}

fn dump_search_parameters(options: &SearchOptions) {
    println!("name: {:?}", options.name);
    println!("path: {:?}", options.path);
    println!("size: {:?}", options.size);
    println!("date: {:?}", options.date);
    println!("debug: {:?}", options.debug);
}

fn main() {
    let options = parse_command_line();
    if options.debug.is_some() {
        dump_search_parameters(&options);
    }

    let path = Path::new(options.path.as_ref().unwrap()).to_path_buf();

    let mut matchers = Vec::<Box<dyn Matcher>>::new();
    if options.name.is_some() {
        matchers.push(Box::new(FileNameMatcher::new(options.name.unwrap())));
    } else if options.size.is_some() {
        matchers.push(Box::new(FileSizeMatcher::new(options.size.unwrap())));
    } else if options.date.is_some() {
        matchers.push(Box::new(FileDateMatcher::new(options.date.unwrap())));
    }

    let matcher_pipeline = MatcherPipeline::new(matchers);
    traverse(&path, &matcher_pipeline);
}
