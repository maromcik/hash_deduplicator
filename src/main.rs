use std::io;
use crate::hash::process;
mod search;
mod hash;


fn main() -> io::Result<()>{
    process()
}
