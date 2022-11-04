//! Advent of Code 2017 Day 4

// --- Part Two ---
// For added security, yet another system policy has been put in place. Now, a valid passphrase
// must contain no two words that are anagrams of each other - that is, a passphrase is invalid if
// any word's letters can be rearranged to form any other word in the passphrase.
//
// For example:
//
// - abcde fghij is a valid passphrase.
// - abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the
//   first word.
// - a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming
//   another word.
// - iiii oiii ooii oooi oooo is valid.
// - oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other
//   word.
//
// Under this new system policy, how many passphrases are valid?

use std::collections::HashSet;

const LOWERCASE_A_DEC: usize = b'a' as usize;

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
        let mut seen: HashSet<u64> = HashSet::new();

        for word in phrase {
            let hash = prime_hash(word);
            if !seen.insert(hash) {
                continue 'outer;
            }
        }

        valid_count += 1;
    }

    valid_count
}

/// Assumes the incoming str is only composed of lowercase ascii letters.
#[rustfmt::skip]
fn prime_hash(word: &str) -> u64 {
    // First 26 primes.
    const PRIMES: [u64; 26] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
     // a  b  c  d  e   f   g   h   i   j   k   l   m   n   o   p   q   r   s   t   u   v   w   x   y   z
    ];

    word.chars()
        .map(|c| PRIMES[c as usize - LOWERCASE_A_DEC])
        .product()
}

#[test]
fn part2_test() {
    assert_eq!(solve(vec![vec!["aa", "bb", "cc"]]), 1);
    assert_eq!(solve(vec![vec!["ab", "ba"]]), 0);
    assert_eq!(
        solve(vec![
            vec!["ab", "ba"],
            vec!["asdfasdfasdfasdf", "asdffdsaasdffdsa"],
            vec!["aaa", "bbb", "ccc"],
        ]),
        1
    );
}
