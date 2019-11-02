use std::env;
use std::io::{prelude::*,BufReader,BufRead};
use std::fs::File;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        let lines = lines_from_file(&args[1]);
        for line in lines {
            if &line[..4] == "sqrt" {
                //
            }
        }
    } else {
        println!("usage: mathlang [FILE_NAME]");
    }
}