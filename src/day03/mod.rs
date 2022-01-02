use crate::utils::read_file;

pub fn day03() {
    // get lines from file and parse to vec of string.
    // let lines = read_file("inputs/day03.txt");

    // parse the collected vec of string to vec of int
    // let nums: Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();

    println!("Day 03 - Part 1: {}", 1);
    println!("Day 03 - Part 2: {}", 2);
}

fn part1(lines: &[String]) -> usize {
    let bit_width = lines[0].len() - 1; // -1 for new line
    let mut count = vec![0; bit_width];
    let max = lines.len() / 2;
    for line in lines.iter().map(|s| s.chars()) {
        for c in line.enumerate() {
            if c.1 == '1' {
                count[c.0] += 1;
            }
        }
    }
    let mut gamma = 0;
    for value in &count {
        gamma = gamma << 1;
        gamma |= (*value > max) as usize;
    }
    println!("gamma {}", gamma);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let lines = vec![
            "00100\n".to_string(),
            "11110\n".to_string(),
            "10110\n".to_string(),
            "10111\n".to_string(),
            "10101\n".to_string(),
            "01111\n".to_string(),
            "00111\n".to_string(),
            "11100\n".to_string(),
            "10000\n".to_string(),
            "11001\n".to_string(),
            "00010\n".to_string(),
            "01010\n".to_string(),
        ];
        assert_eq!(part1(&lines), 198);
    }
}

