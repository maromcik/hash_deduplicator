use std::{fs, io};
use crate::hash::process;
mod search;
mod hash;

use std::fs::File;
use std::io::{BufWriter, Write};
use clap::Parser;
use crate::search::list_files;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Paths to files
    #[clap(short, long, value_name = "PATH_TO_FILE", value_delimiter = ' ', num_args = 1..)]
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

    let deleted_files = File::create("deleted_files.txt")?;
    let mut log = BufWriter::new(deleted_files);
    let mut sum = 0;
    for (k, v) in duplicates.iter() {
        println!("digest: {:?}, path: {:?}", k, v);

        for f in v.iter().skip(1) {
            write!(log, "{}\n", f)?;
            if cli.delete {
                delete_file(f)?;
            }
            sum += 1;
        }
    }
    println!("Duplicates count: {}", duplicates.len());
    println!("Deleted files: {sum}");
    Ok(())
}

fn delete_file(file: &str) -> io::Result<()> {
    fs::remove_file(file)?;
    println!("{file} deleted");
    Ok(())
}