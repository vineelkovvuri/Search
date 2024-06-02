use clap::{Arg, Command};

use crate::SearchOptions;

pub fn parse_command_line() -> SearchOptions {
    let args = Command::new("search")
        .version("1.0")
        .about("File search program")
        .author("Vineel Kovvuri")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("File name to search"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Path to search")
                .default_value("C:\\"),
        )
        .get_matches();
    let name = args.get_one::<String>("name").expect("name is expected");
    let path = args.get_one::<String>("path").expect("path is expected");

    SearchOptions {
        name: name.to_string(),
        path: path.to_string(),
    }
}
