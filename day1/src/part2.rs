#![feature(test)]
//! Advent of Code 2017 Day 1

// --- Part Two ---
// You notice a progress bar that jumps to 50% completion. Apparently, the door isn't yet
// satisfied, but it did emit a star as encouragement. The instructions change:
//
// Now, instead of considering the next digit, it wants you to consider the digit halfway around
// the circular list. That is, if your list contains 10 items, only include a digit in your sum if
// the digit 10/2 = 5 steps forward matches it. Fortunately, your list has an even number of
// elements.
//
// For example:
//
// 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
// 1221 produces 0, because every comparison is between a 1 and a 2.
// 123425 produces 4, because both 2s match each other, but no other digit has a match.
// 123123 produces 12.
// 12131415 produces 4.
//
// What is the solution to your new captcha?

extern crate test;

fn solve(input: &str) -> u16 {
    // cut the input in half
    let (set_a, set_b) = input.as_bytes().split_at(input.len() / 2);

    let mut sum: u16 = 0;

    for i in 0..set_a.len() {
        if set_a[i] == set_b[i] {
            sum += (set_a[i] * 2 - 96) as u16;
        }
    }

    sum
}

fn main() {
    let input = include_str!("../input");

    println!("{}", solve(input));
}

#[test]
fn solve_test() {
    assert_eq!(solve("1212"), 6);
    assert_eq!(solve("1221"), 0);
    assert_eq!(solve("123425"), 4);
    assert_eq!(solve("123123"), 12);
    assert_eq!(solve("12131415"), 4);
}

#[bench]
fn solve_bench(b: &mut test::Bencher) {
    b.iter(|| solve(include_str!("../input-150MB")));
}
