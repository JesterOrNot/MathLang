extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// This function reads the lines from a file into a vector
pub fn lines_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
}

pub fn main() {
    let args: Vec<_> = env::args().collect(); // Get command line arguments
    if args.len() == 2 {
        let lines = lines_from_file(&args[1]); // Opens the file given by the user
        for mut line in lines {
            if line.len() <= 3 {
                // fixes a wierd index out of bounds error
                &line.push_str("   ");
            }
            if line.contains("//") {
                // Comments!
                let tmp: Vec<&str> = line.split("//").collect(); // Splits the text on comment
                if !tmp[0].is_empty() {
                    // Basically if the first characters in the line
                    // aren't `//` then assign the first item
                    // (the text before the inline comment)
                    // otherwise move on because all the text
                    // is commented out
                    line = tmp[0].to_string();
                } else {
                    continue;
                }
            }
            if &line[..4] == "say " {
                println!("{}", &line[4..]); // This implements the say keyword which prints things to the screen
            } else if &line[..4] == "sqrt" {
                // This impements sqrts
                println!("{}", f64::sqrt(line[5..].parse::<f64>().unwrap()));
            } else if &line[..3] == "log" {
                // This implements logarithms
                let lists: Vec<_> = line.split(" ").collect();
                println!(
                    "{}",
                    lists[1]
                        .parse::<f64>()
                        .unwrap()
                        .log(lists[2].parse::<f64>().unwrap())
                );
            // Implements sin cos and tan
            } else if &line[..3] == "sin" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().sin());
            } else if &line[..3] == "cos" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().cos());
            } else if &line[..3] == "tan" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().tan());
            } else if &line[..4] == "cbrt" {
                // CubeRoots
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().cbrt());
            // Implements asin acos and atan
            } else if &line[..4] == "asin" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().sin().asin());
            } else if &line[..4] == "atan" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().tan().atan());
            } else if &line[..4] == "acos" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().cos().acos());
            } else if line.contains("+") {
                let re = Regex::new(r"(\d*) *\+ *(\d*)").unwrap(); // Use regex to define syntax for addition and capture numbers
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        cap[1].parse::<f64>().unwrap() + cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("%") {
                let re = Regex::new(r"(\d*) *% *(\d*)").unwrap(); // Use regex to define syntax for modulos and capture numbers
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() % &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("-") {
                let re = Regex::new(r"(\d*) *\- *(\d*)").unwrap(); // Use regex to define syntax for subtraction and capture numbers
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() - &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("*") && !line.contains("**") {
                let re = Regex::new(r"(\d*) *\* *(\d*)").unwrap(); // Use regex to define syntax for multiplication and capture numbers
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() * &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("/") {
                let re = Regex::new(r"(\d*) */ *(\d*)").unwrap(); // Use regex to define syntax for division and capture numbers
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() / &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("**") && !line.contains(" * ") {
                // Use regex to define syntax for exponents and capture numbers
                let re = Regex::new(r"(\d*) *\*\* *(\d*)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        cap[1]
                            .parse::<f64>()
                            .unwrap()
                            .powf(cap[2].parse::<f64>().unwrap())
                    );
                }
            } else {
                println!("Syntax Error!"); // If nothing passes pretty much assume its a syntax error
            }
        }
    } else {
        println!("usage: mathlang [FILE_NAME]");
    }
}
