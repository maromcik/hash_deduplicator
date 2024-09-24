use std::{fs, io};
use crate::hash::process;
mod search;
mod hash;
use std::env;
use crate::search::list_files;

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let paths = args.into_iter().skip(1).collect::<Vec<String>>();
    println!("Looking in paths:");
    for p in &paths {
        println!("{p}");
    }
    let files = list_files(paths)?;
    let duplicates = process(files)?;

    for (k, v) in duplicates.iter() {
        println!("digest: {:?}, path: {:?}", k, v);
        let _ = v
            .into_iter()
            .skip(1)
            .map(|f| delete_file(f))
            .collect::<Vec<io::Result<()>>>();
    }

    println!("Duplicates count: {}", duplicates.len());

    Ok(())
}

fn delete_file(file: &str) -> io::Result<()> {
    fs::remove_file(file)?;
    println!("{file} deleted");
    Ok(())
}