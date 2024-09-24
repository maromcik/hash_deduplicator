use std::{fs, io};
use crate::hash::process;
mod search;
mod hash;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::search::list_files;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let paths = args.into_iter().skip(1).collect::<Vec<String>>();
    println!("Looking in paths:");
    for p in &paths {
        println!("{p}");
    }
    let files = list_files(paths)?;
    let duplicates = process(files)?;

    let deleted_files = File::create("deleted_files.txt")?;
    let mut log = BufWriter::new(deleted_files);
    let mut sum = 0;
    for (k, v) in duplicates.iter() {
        println!("digest: {:?}, path: {:?}", k, v);

        for f in v.iter().skip(1) {
            write!(log, "{}\n", f)?;
            delete_file(f)?;
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