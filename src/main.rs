use std::path::Path;

use cmd::parse_command_line;
use matchers::filenamematcher::FileNameMatcher;
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
    name: String,
    path: String,
    debug: bool,
}

fn dump_search_parameters(options: &SearchOptions) {
    println!("name: {}", options.name);
    println!("path: {}", options.path);
    println!("debug: {}", options.debug);
}

fn main() {
    let options = parse_command_line();
    if options.debug {
        dump_search_parameters(&options);
    }

    let path = Path::new(&options.path).to_path_buf();

    let filenamematcher = FileNameMatcher::new(options.name.clone());

    search2(&path, &filenamematcher);
}
