use crate::hash::{process, ProcessedFile};
use std::collections::HashMap;
use std::{fs, io};
mod hash;
mod search;

use crate::search::list_files;
use clap::Parser;
use md5::Digest;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Paths to files
    #[clap(short, long, value_name = "PATH_TO_FILE", num_args = 1..)]
    paths: Vec<String>,

    /// delete files
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    delete: bool,

    /// verbose files
    #[arg(short = 'v', long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    println!("Looking in paths:");
    for p in &cli.paths {
        println!("{p}");
    }
    let files = list_files(cli.paths)?;
    let duplicates = process(files, cli.verbose)?;
    let mut total_size = 0;
    for (k, v) in duplicates.iter() {
        println!("digest: {:?}, path: {:?}", k, v.iter().map(|f| &f.path).collect::<Vec<&PathBuf>>());
        total_size += v.iter().skip(1).map(|f| f.size).sum::<u64>();
    }
    println!("Duplicates count: {}", duplicates.len());
    println!("Duplicates size: {} MB", total_size / 1024 / 1024);

    if cli.delete && !duplicates.is_empty() {
        println!("Do you really want to delete all duplicates (keep the first file in the list) y/n: ");
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        buffer = buffer.to_lowercase().trim().to_string();
        if buffer == "y" || buffer == "yes" {
            delete(duplicates)?;
        }
    }

    Ok(())
}

fn delete(duplicates: HashMap<Digest, Vec<ProcessedFile>>) -> io::Result<()> {
    let deleted_files = File::create("deleted_files.txt")?;
    let mut log = BufWriter::new(deleted_files);
    let mut sum = 0;
    let mut total_size = 0;
    for (k, v) in duplicates.iter() {
        println!("digest: {:?}, path: {:?}", k, v.iter().map(|f| &f.path).collect::<Vec<&PathBuf>>());

        for f in v.iter().skip(1) {
            write!(log, "{:?}\n", f.path)?;
            delete_file(&f.path)?;
            total_size += f.size;
            sum += 1;
        }
    }
    println!("Deleted files: {sum}");
    println!("Freed space: {} MB", total_size / 1024 / 1024);

    Ok(())
}

fn delete_file(file: &PathBuf) -> io::Result<()> {
    fs::remove_file(file)?;
    println!("{} deleted", file.to_str().unwrap_or_default());
    Ok(())
}
