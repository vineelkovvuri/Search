use clap::{Arg, Command};

use crate::{matchers::filedatematcher::FileDate, SearchOptions};

fn parse_date_param(date_str: &str) -> Result<(FileDate, FileDate), String> {
    let date_str = date_str.trim();
    let mut splits = date_str.split(',');
    let date_from_str = splits.next().ok_or("Failed to parse the date string")?;
    let date_to_str = splits.next().ok_or("Failed to parse the date string")?;

    let splits: Vec<&str> = date_from_str.split('-').collect();
    if splits.len() != 3 {
        return Err("Input string not in correct format".to_string());
    }

    let year_from = splits[0]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;
    let month_from = splits[1]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;
    let day_from = splits[2]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;

    let splits: Vec<&str> = date_to_str.split('-').collect();
    if splits.len() != 3 {
        return Err("Input string not in correct format".to_string());
    }

    let year_to = splits[0]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;
    let month_to = splits[1]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;
    let day_to = splits[2]
        .parse::<u64>()
        .map_err(|_| "Input string not in correct format")? as u16;

    // DateTime::parse_from_str(date_from_str, "%Y-%m-%d")
    Ok((
        FileDate(year_from, month_from, day_from),
        FileDate(year_to, month_to, day_to),
    ))
}

fn convert_to_bytes(size_str: &str) -> Result<u64, String> {
    let len = size_str.len();
    if len <= 2 {
        return Ok(size_str.parse::<u64>().unwrap_or(0));
    }

    let suffix = &size_str[len - 2..].to_ascii_uppercase();
    let last = suffix.as_str();
    let remaining = &size_str[..len - 2];

    let multiplier = match last {
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        "TB" => 1024 * 1024 * 1024 * 1024,
        _ => 1,
    };

    remaining
        .parse::<u64>()
        .map(|rem| rem * multiplier)
        .map_err(|_| format!("Failed to convert the {}", size_str))
}

fn parse_size_param(size_str: &str) -> Result<(u64, u64), String> {
    let size_str = size_str.trim();
    let mut splits = size_str.split(',');
    let size_min_str = splits.next().or(Some("0"));
    let size_max_str = splits.next().or(Some("0"));

    let size_min_bytes = convert_to_bytes(size_min_str.unwrap().trim())?;
    let size_max_bytes = convert_to_bytes(size_max_str.unwrap().trim())?;

    Ok((size_min_bytes, size_max_bytes))
}

pub fn parse_command_line() -> SearchOptions {
    let matches = Command::new("search")
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
                .help("Size of the file to search. Specified as -s \"10MB,20MB\""),
        )
        .arg(
            Arg::new("date")
                .short('d')
                .long("date")
                .help("Date of the file to search. Specified as -s \"2024-01-20,2024-03-22\""),
        )
        .arg(
            Arg::new("debug")
                .short('x')
                .required(false)
                .help("Dump search parameters"),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").map(|v| v.to_string());
    let path = matches.get_one::<String>("path").map(|v| v.to_string());
    let debug = matches.get_one::<bool>("debug").copied();
    let size_str = matches.get_one::<String>("size").map(|v| v.as_str());
    let size = parse_size_param(size_str.unwrap_or("0,0")).ok();
    let date_str = matches.get_one::<String>("size").map(|v| v.as_str());
    let date = parse_date_param(date_str.unwrap_or("0000-00-00,0000-00-00")).ok();

    SearchOptions {
        name,
        path,
        debug,
        size,
        date,
    }
}

#[test]
fn size_str_test() {
    assert_eq!(parse_size_param("10KB,20KB"), Ok((10 * 1024, 20 * 1024)));
    assert_eq!(parse_size_param("10KB, 20KB"), Ok((10 * 1024, 20 * 1024)));
    assert_eq!(
        parse_size_param("10KB,  20KB  "),
        Ok((10 * 1024, 20 * 1024))
    );
    assert_eq!(parse_size_param(", 20KB"), Ok((0, 20 * 1024)));
    assert_eq!(parse_size_param("10KB,"), Ok((10 * 1024, 0)));
    assert_eq!(parse_size_param("10KB,20KB"), Ok((10 * 1024, 20 * 1024)));
}
