//The code is not complete and has to be made some modifications but the logic looks clear.


use std::env;
use std::fs;
use std::fs::read_link;
//use std::os::unix::fs;
//use std::path::{Path,PathBuf};

fn main() -> std::io::Result<()> {
    let args: Vec<String>= env::args().collect();
    let mut f = fs::read_link(&args[1])?;
    //println!("{}",f.display());
    //let mut buf_reader = BufReader::new(f);
    /*let mut contents = String::new();
    f.read_to_string(&mut contents)?;*/
    fs::symlink(f, &args[2])?;
    Ok(())
}
