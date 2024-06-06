use std::path::Path;

use cmd::parse_command_line;
use matchers::{filenamematcher::FileNameMatcher, filesizematcher::FileSizeMatcher};
use search::search2;

mod cmd;
mod matchers;
mod search;

struct SearchOptions {
    name: Option<String>,
    path: Option<String>,
    debug: Option<bool>,
    size: Option<(u64, u64)>,
}

fn dump_search_parameters(options: &SearchOptions) {
    println!("name: {}", options.name.as_ref().unwrap_or(&"".to_string()));
    println!("path: {}", options.path.as_ref().unwrap_or(&"".to_string()));
    println!("size: {:?}", options.size.unwrap_or((0, 0)));
    println!("debug: {}", options.debug.as_ref().unwrap_or(&false));
}

fn main() {
    let options = parse_command_line();
    if options.debug.is_some() {
        dump_search_parameters(&options);
    }

    let path = Path::new(options.path.as_ref().unwrap()).to_path_buf();

    if options.name.is_some() {
        let matcher = FileNameMatcher::new(options.name.as_ref().unwrap().clone());
        search2(&path, &matcher);
    } else if options.size.is_some() {
        let matcher = FileSizeMatcher::new(options.size.unwrap());
        search2(&path, &matcher);
    }
}
