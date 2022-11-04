//! Advent of Code 2017 Day 4

// --- Day 4: High-Entropy Passphrases ---
//
// A new system policy has been put in place that requires all accounts to use a passphrase instead
// of simply a password. A passphrase consists of a series of words (lowercase letters) separated
// by spaces.
//
// To ensure security, a valid passphrase must contain no duplicate words.
//
// For example:
//
//   - aa bb cc dd ee is valid.
//   - aa bb cc dd aa is not valid - the word aa appears more than once.
//   - aa bb cc dd aaa is valid - aa and aaa count as different words.
//
// The system's full passphrase list is available as your puzzle input. How many passphrases are
// valid?

use std::collections::HashSet;

fn main() {
    let input = parse(include_str!("../input"));

    println!("{}", solve(input));
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

/// Calculate number of valid passphrases.
fn solve(passphrases: Vec<Vec<&str>>) -> u32 {
    let mut valid_count = 0;

    'outer: for phrase in passphrases.iter() {
        let mut seen: HashSet<&str> = HashSet::new();

        for word in phrase {
            if !seen.insert(word) {
                continue 'outer;
            }
        }

        valid_count += 1;
    }

    valid_count
}

#[test]
fn part1_test() {
    assert_eq!(
        solve(vec![
            vec!["aa", "bb", "cc"],
            vec!["aa", "aa", "cc"],
            vec!["cc", "bb", "aa"],
        ]),
        2
    );
}
