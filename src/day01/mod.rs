use crate::utils::read_file;

pub fn day01() {
    // get lines from file and parse to vec of string.
    let lines = read_file("inputs/day01.txt");

    // parse the collected vec of string to vec of int
    let nums: Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();

    // compare values i < i + 1 and sum count if it's true.
    println!("Day 01 - Part 1: {}", increasing(&nums, 1));
    println!("Day 01 - Part 2: {}", increasing(&nums, 3));
}

fn increasing(nums: &[i32], offset: usize) -> usize {
    nums.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum()
}