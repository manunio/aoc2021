use crate::utils::read_file;

pub fn day02() {
    // get lines from file and parse to vec of string.
    let lines = read_file("inputs/day02.txt");

    println!("Day 02 - Part 1: {}", part1(&lines));
    println!("Day 02 - Part 2: {}", part2(&lines));
}

fn part1(commands: &[String]) -> i32 {
    //  (depth, horizontal position)
    let result: (i32, i32) = commands.iter()
        .map(|s| s.split_whitespace())
        .map(|mut cmd| match cmd.next().unwrap() {
            "forward" => (0, cmd.next().unwrap().parse().unwrap()),
            "down" => (cmd.next().unwrap().parse().unwrap(), 0),
            "up" => (-(cmd.next().unwrap().parse::<i32>().unwrap()), 0),
            x => panic!("unknown command {}", x),
        }).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    result.0 * result.1
}

fn part2(commands: &[String]) -> i32 {
    // (aim, horizontal position, depth)
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;

    for cmd in commands {
        let mut x = cmd.split_whitespace();
        let dir = x.next().unwrap();
        let val = x.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "up" => aim -= val,
            "down" => aim += val,
            "forward" => {
                hpos += val;
                depth += aim * val;
            }
            x => panic!("unknown command {}", x),
        };
    }

    hpos * depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position() {
        let commands = vec![
            "forward 5\n".to_string(),
            "down 5\n".to_string(),
            "forward 8\n".to_string(),
            "up 3\n".to_string(),
            "down 8\n".to_string(),
            "forward 2\n".to_string(),
        ];
        assert_eq!(part1(&commands), 150);
        assert_eq!(part2(&commands), 900);
    }
}