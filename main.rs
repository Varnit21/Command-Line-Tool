use clap::{App, Arg};
use colored::Colorize;
use humansize::{file_size_opts as options, FileSize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct FileInfo {
    name: String,
    size: Option<u64>,
    modified: Option<i64>,
    is_dir: bool,
    permissions: Option<String>,
    owner: Option<String>,
    group: Option<String>,
}

impl FileInfo {
    fn new(
        name: String,
        size: Option<u64>,
        modified: Option<i64>,
        is_dir: bool,
        permissions: Option<String>,
        owner: Option<String>,
        group: Option<String>,
    ) -> Self {
        FileInfo {
            name,
            size,
            modified,
            is_dir,
            permissions,
            owner,
            group,
        }
    }
}

fn handle_error(message: &str) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn get_permissions(metadata: &fs::Metadata) -> Option<String> {
    metadata
        .permissions()
        .mode()
        .to_string()
        .get(2..)
        .map(|s| format!("{:03}", usize::from_str_radix(s, 8).unwrap()))
}

fn get_owner(metadata: &fs::Metadata) -> Option<String> {
    metadata.uid().to_string().parse::<String>().ok()
}

fn get_group(metadata: &fs::Metadata) -> Option<String> {
    metadata.gid().to_string().parse::<String>().ok()
}

fn explore_directory(
    dir_path: &str,
    show_hidden: bool,
    sort_by: &str,
    filter_by: Option<&str>,
    recursive: bool,
) -> Vec<FileInfo> {
    let mut files: Vec<FileInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            if !show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            let size = metadata.len();
            let modified =
                metadata.modified().ok()?.duration_since(std::time::SystemTime::UNIX_EPOCH).ok()?.as_secs() as i64;
            let permissions = get_permissions(&metadata);
            let owner = get_owner(&metadata);
            let group = get_group(&metadata);
            let is_dir = metadata.is_dir();

            files.push(FileInfo::new(name, Some(size), Some(modified), is_dir, permissions, owner, group));

            if recursive && is_dir {
                let subdir_path = Path::new(dir_path).join(entry.file_name());
                let subdir_files =
                    explore_directory(&subdir_path.to_string_lossy(), show_hidden, sort_by, filter_by, recursive);
                files.extend(subdir_files);
            }
        }
    }

    // Sorting
    match sort_by {
        "size" => files.sort_by(|a, b| a.size.unwrap_or(0).cmp(&b.size.unwrap_or(0))),
        "date" => files.sort_by(|a, b| a.modified.unwrap_or(0).cmp(&b.modified.unwrap_or(0))),
        _ => files.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    // Filtering
    if let Some(extension) = filter_by {
        files.retain(|file| file.name.ends_with(extension));
    }

    files
}

fn perform_file_operation(operation: &str, source: &str, destination: &str) {
    let command = match operation {
        "copy" => Command::new("cp").arg("-r").arg(source).arg(destination),
        "move" => Command::new("mv").arg(source).arg(destination),
        "delete" => Command::new("rm").arg("-r").arg(source),
        _ => return,
    };

    let result = command.status();

    if let Err(e) = result {
        handle_error(&format!("Failed to perform file operation: {}", e));
    }
}

fn view_file(file_path: &str) {
    let command = match std::env::consts::OS {
        "windows" => Command::new("notepad.exe").arg(file_path),
        "macos" | "linux" => Command::new("cat").arg(file_path),
        _ => return,
    };

    let result = command.status();

    if let Err(e) = result {
        handle_error(&format!("Failed to view file: {}", e));
    }
}

fn edit_file(file_path: &str) {
    let command = match std::env::consts::OS {
        "windows" => Command::new("notepad.exe").arg(file_path),
        "macos" => Command::new("open").arg("-e").arg(file_path),
        "linux" => Command::new("xdg-open").arg(file_path),
        _ => return,
    };

    let result = command.status();

    if let Err(e) = result {
        handle_error(&format!("Failed to edit file: {}", e));
    }
}

fn main() {
    let matches = App::new("Rust File Explorer")
        .version("1.0.1")
        .author("Varnit21")
        .about("A command-line file explorer in Rust")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("dir")
                .value_name("DIRECTORY")
                .help("Sets the directory to explore")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .value_name("SORT")
                .help("Sort files by name, size, or modification date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filter")
                .short("f")
                .long("filter")
                .value_name("FILTER")
                .help("Filter files by extension")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hidden")
                .short("h")
                .long("hidden")
                .help("Show hidden files and directories"),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Explore directories recursively"),
        )
        .arg(
            Arg::with_name("operation")
                .short("o")
                .long("operation")
                .value_name("OPERATION")
                .help("Perform file operation: copy, move, delete")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("source")
                .short("src")
                .long("source")
                .value_name("SOURCE")
                .help("Source file or directory for file operation")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("dest")
                .long("destination")
                .value_name("DESTINATION")
                .help("Destination for file operation")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("view")
                .short("v")
                .long("view")
                .value_name("VIEW")
                .help("View the content of a file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .value_name("EDIT")
                .help("Edit the content of a file")
                .takes_value(true),
        )
        .get_matches();

    if let Some(operation) = matches.value_of("operation") {
        if let (Some(source), Some(destination)) = (matches.value_of("source"), matches.value_of("destination")) {
            perform_file_operation(operation, source, destination);
            return;
        }
    }

    if let Some(file_path) = matches.value_of("view") {
        view_file(file_path);
        return;
    }

    if let Some(file_path) = matches.value_of("edit") {
        edit_file(file_path);
        return;
    }

    let dir_path = matches.value_of("directory").unwrap_or(".");
    let show_hidden = matches.is_present("hidden");
    let sort_by = matches.value_of("sort").unwrap_or("name");
    let filter_by = matches.value_of("filter");
    let recursive = matches.is_present("recursive");

    let files = explore_directory(dir_path, show_hidden, sort_by, filter_by, recursive);

    for file in files {
        let display_name = if file.is_dir {
            file.name.blue().to_string()
        } else {
            file.name.white().to_string()
        };

        let size = if let Some(size) = file.size {
            format!("{}", size.file_size(options::CONVENTIONAL).unwrap())
        } else {
            String::from("N/A")
        };

        let modified = if let Some(modified) = file.modified {
            chrono::NaiveDateTime::from_timestamp(modified, 0).to_string()
        } else {
            String::from("N/A")
        };

        let permissions = if let Some(permissions) = &file.permissions {
            permissions.to_string()
        } else {
            String::from("N/A")
        };

        let owner = if let Some(owner) = &file.owner {
            owner.to_string()
        } else {
            String::from("N/A")
        };

        let group = if let Some(group) = &file.group {
            group.to_string()
        } else {
            String::from("N/A")
        };

        println!(
            "{:<30} {:<15} {:<20} {:<20} {:<20} {:<20} {}",
            display_name, size, modified, permissions, owner, group
        );
    }
}
