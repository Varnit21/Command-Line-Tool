use std::env;
use std::fs;
use std::path::PathBuf;
use clap::{App, Arg};
use chrono::offset::Local;
use chrono::DateTime;
use colored::*;
use humanize::FileSize;
use users::{get_user_by_uid, get_group_by_gid};

fn main() {
    let matches = App::new("File Explorer")
        .version("1.0")
        .author("Your Name")
        .about("A professional file exploration tool")
        .arg(Arg::with_name("directory")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .help("Sets the directory to explore")
            .takes_value(true))
        .arg(Arg::with_name("sort")
            .short("s")
            .long("sort")
            .value_name("SORT")
            .possible_values(&["name", "size", "date"])
            .help("Sort files by name, size, or date")
            .takes_value(true))
        .arg(Arg::with_name("filter")
            .short("f")
            .long("filter")
            .value_name("FILTER")
            .help("Filter files by extension")
            .takes_value(true))
        .arg(Arg::with_name("hidden")
            .short("h")
            .long("hidden")
            .help("Show hidden files and directories"))
        .arg(Arg::with_name("human-readable")
            .short("hr")
            .long("human-readable")
            .help("Display human-readable file sizes"))
        .arg(Arg::with_name("details")
            .short("dt")
            .long("details")
            .help("Display file details (permissions, owner, etc.)"))
        .get_matches();

    let dir_path = matches.value_of("directory").unwrap_or(".");
    let entries = fs::read_dir(dir_path).expect("Failed to read directory");
    let mut files: Vec<(String, u64, DateTime<Local>)> = Vec::new();
    let mut total_size: u64 = 0;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

            if !matches.is_present("hidden") && file_name.starts_with('.') {
                continue;
            }

            if path.is_file() {
                let metadata = fs::metadata(&path).expect("Failed to read metadata");
                let file_size = metadata.len();
                let file_date = DateTime::from(metadata.modified().expect("Failed to get modification time"));

                total_size += file_size;

                files.push((file_name, file_size, file_date));
            }
        }
    }

    if let Some(sort_type) = matches.value_of("sort") {
        match sort_type {
            "name" => files.sort_by(|a, b| a.0.cmp(&b.0)),
            "size" => files.sort_by(|a, b| a.1.cmp(&b.1)),
            "date" => files.sort_by(|a, b| a.2.cmp(&b.2)),
            _ => (),
        }
    }

    if let Some(filter_ext) = matches.value_of("filter") {
        files.retain(|(name, _, _)| name.ends_with(filter_ext));
    }

    for (name, size, date) in files {
        let formatted_size = if matches.is_present("human-readable") {
            format!("{:>12}", size.file_size(humanize::FileType::BINARY).unwrap())
        } else {
            size.to_string()
        };

        let file_details = if matches.is_present("details") {
            let metadata = fs::metadata(&name).expect("Failed to read file metadata");
            let permissions = metadata.permissions();
            let owner_uid = metadata.uid();
            let owner_name = get_user_by_uid(owner_uid).map_or_else(|| owner_uid.to_string(), |user| user.name().to_string_lossy().to_string());
            let group_gid = metadata.gid();
            let group_name = get_group_by_gid(group_gid).map_or_else(|| group_gid.to_string(), |group| group.name().to_string_lossy().to_string());

            format!(
                "Permissions: {:?}, Owner: {}, Group: {}",
                permissions, owner_name, group_name
            )
        } else {
            String::new()
        };

        if name.ends_with(".rs") {
            println!("Name: {:<20} Size: {:>12} Date: {} {}", name.green(), formatted_size.green(), date.format("%Y-%m-%d %H:%M:%S"), file_details);
        } else if name.ends_with(".toml") {
            println!("Name: {:<20} Size: {:>12} Date: {} {}", name.blue(), formatted_size.blue(), date.format("%Y-%m-%d %H:%M:%S"), file_details);
        } else {
            println!("Name: {:<20} Size: {:>12} Date: {} {}", name, formatted_size, date.format("%Y-%m-%d %H:%M:%S"), file_details);
        }
    }

    println!("Total Size: {}", total_size.file_size(humanize::FileType::BINARY).unwrap());
}
