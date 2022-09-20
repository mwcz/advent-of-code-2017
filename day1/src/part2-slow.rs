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

use peekmore::PeekMore;

fn char_to_u32(c: char) -> u32 {
    (c as u32) - 48
}

fn solve(input: &str) -> u32 {
    // create an iterator over the input with nth peeking and wraparound
    let mut digits = input.chars().map(char_to_u32).cycle().peekmore();

    let mut sum = 0;
    let peek_dist = input.len() / 2 - 1; // -1 because we'll peek ahead _after_ advancing the iterator

    for _ in 0..input.len() {
        let n = digits.next();
        if n.as_ref() == digits.peek_nth(peek_dist) {
            sum += n.unwrap();
        }
    }

    sum
}

fn main() {
    let input = include_str!("../input2");

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
