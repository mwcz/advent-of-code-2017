//! A solution to day 1 year .
//! https://adventofcode.com//day/1

// --- Part One ---
//
// You're standing in a room with "digitization quarantine" written in LEDs along one wall. The
// only door is locked, but it includes a small interface. "Restricted Area - Strictly No Digitized
// Users Allowed."
//
// It goes on to explain that you may only leave by solving a captcha to prove you're not a human.
// Apparently, you only get one millisecond to solve the captcha: too fast for a normal human, but
// it feels like hours to you.
//
// The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of
// all digits that match the next digit in the list. The list is circular, so the digit after the
// last digit is the first digit in the list.
//
// For example:
//
// 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the
//      third digit (2) matches the fourth digit.
// 1111 produces 4 because each digit (all 1) matches the next.
// 1234 produces 0 because no digit matches the next.
// 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
// What is the solution to your captcha?

type Model = (ModelPart1, ModelPart2);
type ModelPart1 = Vec<u32>;
type ModelPart2 = String;
type Answer = u32;

pub fn parse(input: String) -> Model {
    (input.trim().chars().map(char_to_u32).collect(), input)
}

pub fn part1((digits, _): Model) -> Answer {
    let mut sum = 0;

    for pair in digits.windows(2) {
        if pair[0] == pair[1] {
            sum += pair[0];
        }
    }

    if digits.first() == digits.last() {
        sum += digits.first().unwrap();
    }

    sum
}

pub fn part2((_, input): Model) -> u16 {
    // cut the input in half
    let (p1, p2) = input.as_bytes().split_at(input.len() / 2);

    let mut sum: u16 = 0;

    for i in 0..p1.len() {
        if p1[i] == p2[i] {
            sum += (p1[i] * 2 - 96) as u16;
        }
    }

    sum
}

fn char_to_u32(c: char) -> u32 {
    (c as u32).saturating_sub(48)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d1");

    #[test]
    fn d1p1_example_test() {
        assert_eq!(part1(parse("1122".to_string())), 3);
        assert_eq!(part1(parse("1111".to_string())), 4);
        assert_eq!(part1(parse("1234".to_string())), 0);
        assert_eq!(part1(parse("91212129".to_string())), 9);
    }

    #[test]
    fn d1p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1393);
    }

    #[test]
    fn d1p2_example_test() {
        assert_eq!(part2(parse("1212".to_string())), 6);
        assert_eq!(part2(parse("1221".to_string())), 0);
        assert_eq!(part2(parse("123425".to_string())), 4);
        assert_eq!(part2(parse("123123".to_string())), 12);
        assert_eq!(part2(parse("12131415".to_string())), 4);
    }

    #[test]
    fn d1p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 1292);
    }
}
