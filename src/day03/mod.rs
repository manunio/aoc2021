use crate::utils::read_file;

pub fn day03() {
    // get lines from file and parse to vec of string.
    let lines = read_file("inputs/day03.txt");

    println!("Day 03 - Part 1: {}", part1(&lines));
    println!("Day 03 - Part 2: {}", "x");
}

fn part1(lines: &[String]) -> usize {
    let bit_width = lines[0].len();
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
    let epsilon = ( ( 1 << bit_width ) - 1 ) ^ gamma;
    epsilon * gamma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let lines = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(part1(&lines), 198);
    }
}

