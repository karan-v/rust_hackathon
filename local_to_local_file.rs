use std::fs;
use std::io::Write;
use std::fs::File;
extern crate walkdir;
use std::env;
use std::io::prelude::*;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::BufReader;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let args: Vec<String>= env::args().collect();
    let file = File::open("/home/karan/Documents/foo.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut f = File::create("/home/karan/Desktop/foo.txt")?;
    let mut f1 = BufWriter::new(f);
    f1.write(contents.as_bytes()).unwrap();
    Ok(())
}

/*************************************
NOTE: the command in terminal will be :-
          cargo run filename1 filename2
**************************************/
