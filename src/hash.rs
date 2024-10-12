use md5;
use md5::Digest;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::path::PathBuf;


pub fn calculate_hash(path: &PathBuf, verbose: bool) -> io::Result<Digest> {
    if verbose {
        println!("Calculating hash: {:?}", path);
    }
    let f = File::open(path)?;
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
    Ok(context.compute())
}


pub fn process(files: Vec<PathBuf>, verbose: bool) -> io::Result<HashMap<Digest, Vec<String>>> {
    let hashes = files
        .par_iter()
        .map(|p| calculate_hash(&p, verbose))
        .filter_map(|h| h.ok())
        .collect::<Vec<Digest>>();

    let mut duplicates: HashMap<Digest, Vec<String>> = HashMap::new();
    for (hash, path) in zip(hashes.into_iter(), files.into_iter()) {
        let str_path = path.into_os_string().into_string().unwrap();
        if duplicates.contains_key(&hash) {
            let k = duplicates.get_mut(&hash).unwrap();
            k.push(str_path);
        } else {
            duplicates.insert(hash, vec![str_path]);
        }
    }

    duplicates.retain(|_, v| v.len() > 1);

    Ok(duplicates)
}

