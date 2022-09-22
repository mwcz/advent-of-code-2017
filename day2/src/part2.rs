//! Advent of Code 2017 Day 2

// --- Part Two ---
// "Great work; looks like we're on the right track after all. Here's a star for your effort."
// However, the program seems a little worried. Can programs be worried?
//
// "Based on what we're seeing, it looks like all the User wanted is some information about the
// evenly divisible values in the spreadsheet. Unfortunately, none of us are equipped for that kind
// of calculation - most of us specialize in bitwise operations."
//
// It sounds like the goal is to find the only two numbers in each row where one evenly divides the
// other - that is, where the result of the division operation is a whole number. They would like
// you to find those numbers on each line, divide them, and add up each line's result.
//
// For example, given the following spreadsheet:
//
//  5 9 2 8
//  9 4 7 3
//  3 8 6 5
//
//  - In the first row, the only two numbers that evenly divide are 8 and 2; the result of this
//    division is 4.
//  - In the second row, the two numbers are 9 and 3; the result is 3.
//  - In the third row, the result is 2.
//  - In this example, the sum of the results would be 4 + 3 + 2 = 9.
//
// What is the sum of each row's result in your puzzle input?

use itertools::Itertools;

fn solve(input: &str) -> u16 {
    let lines = parse(input);

    let mut sum = 0;

    for line in lines {
        for pair in line.iter().permutations(2) {
            let div = pair[0] / pair[1];
            let rem = pair[0] % pair[1];
            if rem == 0 {
                sum += div;
            }
        }
    }

    // no itertools version:
    //
    // for line in lines {
    //     'line: for a in &line {
    //         for b in &line {
    //             if a == b {
    //                 break;
    //             }
    //             let (div_a, rem_a) = (a / b, a % b);
    //             let (div_b, rem_b) = (b / a, b % a);
    //             if rem_a == 0 {
    //                 sum += div_a;
    //                 break 'line;
    //             }
    //             if rem_b == 0 {
    //                 sum += div_b;
    //                 break 'line;
    //             }
    //         }
    //     }
    // }

    sum
}

fn parse(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|line| -> Vec<u16> {
            line.split_whitespace()
                .map(|num| num.parse::<u16>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let input = include_str!("../input");

    println!("{}", solve(input));
}

#[test]
fn solve_test() {
    assert_eq!(
        solve(
            "5 9 2 8
             9 4 7 3
             3 8 6 5"
        ),
        9
    );
}
