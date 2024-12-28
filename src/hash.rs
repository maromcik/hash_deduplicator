use md5;
use md5::Digest;
use rayon::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct ProcessedFile {
    pub path: PathBuf,
    pub hash: Digest,
    pub size: u64,
}

pub fn calculate_hash(path: PathBuf, verbose: bool) -> io::Result<ProcessedFile> {
    if verbose {
        println!("Calculating hash: {:?}", path);
    }
    let f = File::open(&path)?;
    let len = f.metadata()?.len();
    let buf_len = len.min(1_000_000) as usize;
    let mut buf = BufReader::with_capacity(buf_len, f);
    let mut context = md5::Context::new();
    loop {
        // Get a chunk of the file
        let part = buf.fill_buf()?;
        // If that chunk was empty, the reader has reached EOF
        if part.is_empty() {
            break;
        }
        // Add chunk to the md5
        context.consume(part);
        // Tell the buffer that the chunk is consumed
        let part_len = part.len();
        buf.consume(part_len);
    }
    Ok(ProcessedFile {
        path,
        hash: context.compute(),
        size: len,
    })
}

pub fn process(files: Vec<PathBuf>, verbose: bool) -> io::Result<HashMap<Digest, Vec<ProcessedFile>>> {
    let processed_files = files
        .into_par_iter()
        .map(|p| calculate_hash(p, verbose))
        .filter_map(|h| h.ok())
        .collect::<Vec<ProcessedFile>>();

    let mut duplicates: HashMap<Digest, Vec<ProcessedFile>> = HashMap::new();
    for processed_file in processed_files.into_iter() {

        match duplicates.entry(processed_file.hash) {
            Entry::Vacant(e) => { e.insert(vec![processed_file]); },
            Entry::Occupied(mut e) => { e.get_mut().push(processed_file); },
        };

    }

    duplicates.retain(|_, v| v.len() > 1);

    duplicates.iter_mut().for_each(|(_, v)| v.sort_by(|f1, f2| f2.size.cmp(&f1.size) ));
    Ok(duplicates)
}
