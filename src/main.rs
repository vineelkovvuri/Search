use std::path::Path;

use cmd::parse_command_line;
use matchers::{filenamematcher::FileNameMatcher, filesizematcher::FileSizeMatcher};
use search::search2;

mod cmd;
mod matchers;
mod search;

// struct ExactFileNameMatcher<'a> {
//     file_name: &'a str,
// }

// trait Matcher {
//     fn match_path(&self, file_path: &str) -> bool;
// }

// impl<'a> Matcher for ExactFileNameMatcher<'a> {
//     fn match_path(&self, file_path: &str) -> bool {
//         file_path.ends_with(self.file_name)
//     }
// }

struct SearchOptions {
    name: Option<String>,
    path: Option<String>,
    debug: bool,
    size_min: Option<u64>,
    size_max: Option<u64>,
}

fn dump_search_parameters(options: &SearchOptions) {
    println!("name: {}", options.name.as_ref().unwrap_or(&"".to_string()));
    println!("path: {}", options.path.as_ref().unwrap_or(&"".to_string()));
    println!("size_min: {}", options.size_min.unwrap_or(0));
    println!("size_max: {}", options.size_max.unwrap_or(0));
    println!("debug: {}", options.debug);
}

fn main() {
    let options = parse_command_line();
    if options.debug {
        dump_search_parameters(&options);
    }

    let path = Path::new(options.path.as_ref().unwrap()).to_path_buf();

    if options.name.is_some() {
        let matcher = FileNameMatcher::new(options.name.as_ref().unwrap().clone());
        search2(&path, &matcher);
    } else {
        let matcher = FileSizeMatcher::new(options.size_min.unwrap(), options.size_max.unwrap());
        search2(&path, &matcher);
    }
}
