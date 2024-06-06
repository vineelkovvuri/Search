use clap::{Arg, Command};

use crate::SearchOptions;

fn parse_size_param(size: &str) -> (u64, u64) {
    let mut sizes = size.split(',');
    let min = sizes.next().unwrap_or("0").parse::<u64>().unwrap();
    let max = sizes.next().unwrap_or("0").parse::<u64>().unwrap();
    (min, max)
}

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
                .default_value(".")
                .help("Path to search"),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("Size to search"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .required(false)
                .help("Dump search parameters"),
        )
        .get_matches();
    let name = args.get_one::<String>("name").expect("name is expected");
    let path = args.get_one::<String>("path").expect("path is expected");
    let debug = args.get_one::<bool>("debug").unwrap_or(&false);
    let (size_min, size_max) =
        parse_size_param(args.get_one::<String>("size").unwrap_or(&"".to_string()));

    SearchOptions {
        name: Some(name.to_string()),
        path: Some(path.to_string()),
        debug: *debug,
        size_min: Some(size_min),
        size_max: Some(size_max),
    }
}
