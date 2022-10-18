//! Advent of Code 2017 Day 5

// --- Part Two ---
//
// Now, the jumps are even stranger: after each jump, if the offset was three or more, instead
// decrease it by 1. Otherwise, increase it by 1 as before.
//
// Using this rule with the above example, the process now takes 10 steps, and the offset values
// after finding the exit are left as 2 3 2 3 -1.
//
// How many steps does it now take to reach the exit?

fn main() {
    let mut input = parse(include_str!("../input"));

    println!("{}", solve(&mut input));
}

fn parse(input: &str) -> Vec<i16> {
    input
        .trim()
        .lines()
        .map(|num| num.parse::<i16>().expect("invalid number"))
        .collect()
}

/// Calculate number of valid passphrases.
fn solve(jumps: &mut Vec<i16>) -> usize {
    // add 1 to the jump count, even a failed jump counts as a jump
    1 + CPU::new(jumps).count()
}

struct CPU<'a> {
    i: i16,
    arr: &'a mut [i16],
}

impl<'a> CPU<'a> {
    fn new(arr: &'a mut [i16]) -> Self {
        Self { i: 0, arr }
    }
}

impl Iterator for CPU<'_> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let jump = self.arr[self.i as usize];

        let landing = usize::try_from(self.i + jump);

        if let Ok(landing) = landing {
            if self.arr.get(landing).is_some() {
                // increment the visited jump
                if jump >= 3 {
                    self.arr[self.i as usize] -= 1;
                } else {
                    self.arr[self.i as usize] += 1;
                }

                // update current index
                self.i = landing as i16;

                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
fn part2_test() {
    assert_eq!(solve(&mut vec![0, 3, 0, 1, -3]), 10);
}
