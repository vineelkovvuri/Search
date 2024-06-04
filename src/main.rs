use std::path::Path;

use cmd::parse_command_line;
use search::search;

mod cmd;
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

    let mut results: Vec<String> = Vec::new();
    let path = Path::new(&options.path);

    search(&path, &options.name, &mut results);
    for result in results {
        println!("{:?}", result);
    }
}
