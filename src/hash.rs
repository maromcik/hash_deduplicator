use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::ptr::hash;
use md5;
use md5::Digest;
use std::sync::mpsc::Receiver;
use crate::search::list_files;
use rayon::prelude::*;


pub fn calculate_hash(path: &PathBuf) -> Digest {
    let f = File::open(path).unwrap();
    let len = f.metadata().unwrap().len();
    let buf_len = len.min(1_000_000) as usize;
    let mut buf = BufReader::with_capacity(buf_len, f);
    let mut context = md5::Context::new();
    loop {
        // Get a chunk of the file
        let part = buf.fill_buf().unwrap();
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
    context.compute()
}



pub fn aggregator() {
    let files = list_files("/home/roman/Pictures").unwrap();
    let mut ag: HashMap<Digest, Vec<String>> = HashMap::new();

    let hashes = files
        .par_iter()
        .map(|p| calculate_hash(&p))
        .collect::<Vec<Digest>>();



    for (hash, path) in zip(hashes.into_iter(), files.into_iter()) {
        let str_path = path.into_os_string().into_string().unwrap();
        if ag.contains_key(&hash) {
            let k = ag.get_mut(&hash).unwrap();
            k.push(str_path);
        }
        else {
            ag.insert(hash, vec![str_path]);
        }
    }

    for (k, v) in ag.iter() {
        println!("digest: {:?}, path: {:?}", k, v);
    }
 }

