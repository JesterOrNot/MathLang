extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let lines = lines_from_file(&args[1]);
        for line in lines {
            if line.starts_with("//") {
                continue;
            } else if &line[..4] == "sqrt" {
                println!("{}", f64::sqrt(line[5..].parse::<f64>().unwrap()));
            } else if &line[..3] == "log" {
                let lists: Vec<_> = line.split(" ").collect();
                println!(
                    "{}",
                    lists[1]
                        .parse::<f64>()
                        .unwrap()
                        .log(lists[2].parse::<f64>().unwrap())
                );
            } else if &line[..3] == "sin" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().sin());
            } else if &line[..3] == "cos" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().cos());
            } else if &line[..3] == "tan" {
                let lists: Vec<_> = line.split(" ").collect();
                println!("{}", lists[1].parse::<f64>().unwrap().tan());
            } else if line.contains("+") {
                let re = Regex::new(r"(\d*) \+ (\d*)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() + &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("-") {
                let re = Regex::new(r"(\d*) \- (\d*)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() - &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("*") && !line.contains("**") {
                let re = Regex::new(r"(\d*) \* (\d*)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() * &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("/") {
                let re = Regex::new(r"(\d*) / (\d*)").unwrap();
                for cap in re.captures_iter(&line) {
                    println!(
                        "{}",
                        &cap[1].parse::<f64>().unwrap() / &cap[2].parse::<f64>().unwrap()
                    );
                }
            } else if line.contains("**") && !line.contains(" * ") {
                let re = Regex::new(r"(\d*) \*\* (\d*)").unwrap();
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
                println!("Syntax Error!");
            }
        }
    } else {
        println!("usage: mathlang [FILE_NAME]");
    }
}
