//! Advent of Code 2017 Day 6

// https://adventofcode.com/2017/day/6

use heapless::{FnvIndexSet, Vec};

const BANKS: usize = 16;
const SEEN_CAPACITY: usize = 4096;

struct Mem {
    pub banks: MemBanks,
}

type MemBanks = Vec<u8, BANKS>;

impl Mem {
    fn new(banks: MemBanks) -> Self {
        Self { banks }
    }

    fn redistribute(&mut self, i: usize) {
        let block_count = self.banks[i];
        let mut i = i;

        self.banks[i] = 0;

        for _ in 0..block_count {
            // go to next bank
            i = (i + 1) % self.banks.len();

            if let Some(blocks) = self.banks.get_mut(i) {
                *blocks += 1;
            }
        }
    }

    fn position_max_first(&self) -> (usize, u8) {
        // custom max function, since the std max functions all break
        // ties with the last occurrence, not the first
        let mut max_blocks = 0;
        let mut max_pos = 0;

        // of the first.
        for (i, blocks) in self.banks.iter().enumerate() {
            if blocks > &max_blocks {
                max_blocks = *blocks;
                max_pos = i;
            }
        }

        (max_pos, max_blocks)
    }
}

impl From<&str> for Mem {
    fn from(value: &str) -> Self {
        Mem::new(
            value
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }
}

fn main() {
    let input = Mem::from(include_str!("../input"));
    let solution = solve(input);
    println!("{}", solution);
}

/// Calculate number of valid passphrases.
fn solve(mut mem: Mem) -> usize {
    let mut seen = FnvIndexSet::<MemBanks, SEEN_CAPACITY>::new();

    // mark initial layout as "seen"
    seen.insert(mem.banks.clone()).unwrap();

    loop {
        let (max_pos, _) = mem.position_max_first();

        mem.redistribute(max_pos);

        // Could not figure out a way to
        // avoid this clone ----------------------v
        if let Ok(is_new) = seen.insert(mem.banks.clone()) {
            if !is_new {
                break;
            }
        }
    }

    seen.len()
}

#[test]
fn redistribute_test() {
    let mut mem = Mem::from("12 12 14 8 1 0 6 6 5 5 2 2 8 2 16 12");
    mem.redistribute(14);
    assert_eq!(
        mem.banks.to_vec(),
        vec![13, 13, 15, 9, 2, 1, 7, 7, 6, 6, 3, 3, 9, 3, 1, 13]
    );
}

#[test]
fn real_answer_test() {
    let mem = Mem::from("12 12 14 8 1 0 6 6 5 5 2 2 8 2 16 12");
    assert_eq!(solve(mem), 4074);
}
