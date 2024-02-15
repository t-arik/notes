use std::ffi::OsStr;
use std::path;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(PartialEq)]
enum Commands {
    /// does testing things
    List,
    Get,
}

fn main() {
    let cli = Cli::parse();

    if cli.command == Commands::List {
        if let Some(files) = get_pdfs(path::Path::new("sheet-notes")) {
            files
                .iter()
                .filter_map(|x| x.file_name())
                .filter_map(|x| x.to_str())
                .for_each(|x| println!("{x}"));
        } else {
            eprint!("Could not read files in dir");
        }
    }
}

fn get_pdfs(dir: &path::Path) -> Option<Vec<path::PathBuf>> {
    if !dir.is_dir() {
        return None;
    }

    let result: Vec<_> = dir
        .read_dir()
        .unwrap()
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .filter(|entry| entry
                .extension()
                .and_then(OsStr::to_str)
                .eq(&Some("pdf")))
        .collect();
    return Some(result)
}
