use clap::{Arg, ArgAction, Command};

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
                .default_value("kernel32.dll")
                .help("File name to search")
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .default_value(".")
                .help("Path to search")
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .required(false)
                .help("Dump search parameters")
        )
        .get_matches();
    let name = args.get_one::<String>("name").expect("name is expected");
    let path = args.get_one::<String>("path").expect("path is expected");
    let debug = args.get_one::<bool>("debug").or_else(|| Some(&false)).unwrap();

    SearchOptions {
        name: name.to_string(),
        path: path.to_string(),
        debug: *debug,
    }
}
