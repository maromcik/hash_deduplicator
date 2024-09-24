use std::io;
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
    process(files)
}
