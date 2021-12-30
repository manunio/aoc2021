use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day01() {
    // read file and load it to a buffer
    let file = File::open("inputs/day01.txt").expect("file not found");
    let buf = BufReader::new(file);

    // get lines from file and parse to vec of string.
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    // parse the collected vec of string to vec of int
    let nums: Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();
    let mut count = 0;

    // compare values i < i + 1 and increment count if its true.
    for num in nums.windows(2) {
        if num[0] < num[1] {
            count += 1;
        }
    }

    println!("Day 01 - Part 1: {}", count);

    count = 0;

    for i in nums.windows(4) {
        // 0 + 1 + 2 < 1 + 2 + 3
        if i[0] < i[3] {
            count += 1;
        }
    }

    println!("Day 01 - Part 2: {}", count);
}