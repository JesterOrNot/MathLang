use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        for line in BufReader::new(file).lines() {
            println!("{}", line.unwrap());
        }
    } else {
        println!("usage: mathlang [FILE_NAME]");
    }
}