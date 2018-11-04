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

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args[1]=="-r" {
        for entry in WalkDir::new(&args[2]).into_iter().filter_map(|e| e.ok()) {
            let mut out = entry.path();
            let mut output = entry.path().metadata().expect("no");
            let mut flag = 0;
            let mut p1= Path::new(&args[2]).parent().unwrap();
            let mut p3 = out.strip_prefix(p1).expect("ls");
            if output.is_file() {
                let mut p2 = Path::new(&args[3]).join(p3);
                //println!("{}",p2.display());
                    
                    
                let p5 = File::create(p2);
                fs::copy(out,Path::new(&args[3]).join(p3) ).expect("sk");
                
                    
            }
            if output.is_dir() {
                let mut p4 = Path::new(&args[3]).join(p3);
                //println!("{}",p4.display());
                fs::create_dir(p4).expect("dmc");
                    
            }
                
        }
    }
    else {
        println!("Negative");
    }

}
