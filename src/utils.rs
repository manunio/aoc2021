use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file(filename: &str) -> Vec<String> {
    // read file and load it to a buffer
    let file = File::open(filename).expect("file not found");

    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}