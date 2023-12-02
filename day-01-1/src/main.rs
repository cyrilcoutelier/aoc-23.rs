#![warn(clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use aoc2023::process_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(Result::unwrap);

    let result = process_lines(lines);

    println!("The solution is `{}`", result);
}
