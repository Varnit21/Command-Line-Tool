# Rust File Explorer

A professional command-line file exploration tool written in Rust.

## Features
- **Directory Exploration:** Explore and display files in a specified directory.
- **Sorting:** Sort files by name, size, or modification date.
- **Filtering:** Filter files by specifying a file extension.
- **Hidden Files:** Option to show hidden files and directories.
- **Human-Readable Sizes:** Display file sizes in a human-readable format.
- **File Details:** Optionally display file details including permissions, owner, and group.
- **Total Size:** Summarize the total size of displayed files.

## Getting Started
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation
- Clone the repository: git clone https://github.com/your-username/file-explorer.git
- Build the project: cargo build --release
- Add the binary to your PATH: export PATH=$PATH:/path/to/file-explorer/target/release
- 
### Usage
- file-explorer [OPTIONS] [DIRECTORY]

### Arguments
- DIRECTORY: The directory to explore. Defaults to the current directory.
   
### Options
- -d, --dir <DIRECTORY>: Sets the directory to explore
- -s, --sort <SORT>: Sort files by name, size, or date
- -f, --filter <FILTER>: Filter files by extension
- -h, --hidden: Show hidden files and directories
- -hr, --human-readable: Display human-readable file sizes
- -dt, --details: Display file details (permissions, owner, etc.)

### Examples
- Sort files by name: file-explorer -s name
- Sort files by size: file-explorer -s size
- Sort files by date: file-explorer -s date
- Filter files by extension: file-explorer -f .txt
- Show hidden files and directories: file-explorer -h
- Display human-readable file sizes: file-explorer -hr
