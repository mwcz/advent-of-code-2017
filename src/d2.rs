//! A solution to day 2 year .
//! https://adventofcode.com//day/2

type Model = Vec<Vec<u16>>;
type Answer = u16;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .map(|line| -> Vec<u16> {
            line.split_whitespace()
                .map(|num| num.parse::<u16>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(lines: Model) -> Answer {
    let mut sum = 0;

    for line in lines {
        let mut min = u16::MAX;
        let mut max = u16::MIN;
        for num in line {
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }

        if let Some(diff) = max.checked_sub(min) {
            sum += diff
        }
    }

    sum
}

pub fn part2(lines: Model) -> Answer {
    let mut sum = 0;

    // itertools version
    // for line in lines {
    //     for pair in line.iter().permutations(2) {
    //         let div = pair[0] / pair[1];
    //         let rem = pair[0] % pair[1];
    //         if rem == 0 {
    //             sum += div;
    //         }
    //     }
    // }

    for line in lines {
        'line: for a in &line {
            for b in &line {
                if a == b {
                    break;
                }
                let (div_a, rem_a) = (a / b, a % b);
                let (div_b, rem_b) = (b / a, b % a);
                if rem_a == 0 {
                    sum += div_a;
                    break 'line;
                }
                if rem_b == 0 {
                    sum += div_b;
                    break 'line;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d2");
    const EXAMPLE: &str = include_str!("../examples/d2");
    const EXAMPLE2: &str = include_str!("../examples/d2-2");

    #[test]
    fn d2p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 18);
    }

    #[test]
    fn d2p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 45351);
    }

    #[test]
    fn d2p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE2.to_string())), 9);
    }

    #[test]
    fn d2p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 275);
    }
}
