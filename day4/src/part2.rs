//! Advent of Code 2017 Day 3

// --- Day 3: Spiral Memory ---
// --- Part Two ---
// As a stress test on the system, the programs here clear the grid and then store the value 1 in
// square 1. Then, in the same allocation order as shown above, they store the sum of the values in
// all adjacent squares, including diagonals.
//
// So, the first few squares' values are chosen as follows:
//
//  - Square 1 starts with the value 1.
//  - Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
//  - Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
//  - Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
//  - Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
//
// Once a square is written, its value does not change. Therefore, the first few squares would
// receive the following values:
//
//     147  142  133  122   59
//     304    5    4    2   57
//     330   10    1    1   54
//     351   11   23   25   26
//     362  747  806--->   ...
//
// What is the first value written that is larger than your puzzle input?

struct SpiralMemStressTest {
    /// The current X coordinate.
    x: i32,
    /// The current Y coordinate.
    y: i32,
    /// Half the (width or height) of the current square spiral arm, floored.
    radius: i32,
    /// The travel direction for the next iteration.
    next_dir: Direction,
}

impl SpiralMemStressTest {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            radius: 0,
            next_dir: Direction::Right,
        }
    }
}

impl Iterator for SpiralMemStressTest {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let neighbor1 = 1 + (self.x.abs() - 1 + self.y.abs());
        let neighbor2 = 1 + (self.x.abs() + self.y.abs() - 1);
        let this = 1 + (self.x.abs() + self.y.abs());

        let mut dist = this + neighbor1 + neighbor2;

        let at_corner = self.x.abs() == self.radius && self.y.abs() == self.radius;
        if !at_corner {
            dist += 1 + (self.x.abs() - 1 + self.y.abs() - 1);
        }

        // update position and direction
        match self.next_dir {
            Direction::Left => {
                self.x -= 1;

                if self.x == -self.radius {
                    self.next_dir = Direction::Down;
                }
            }
            Direction::Right => {
                self.x += 1;

                // if we just passed the bottom right corner, start a new spiral arm
                if self.x > self.radius {
                    self.radius += 1;
                    self.next_dir = Direction::Up;
                }
            }
            Direction::Up => {
                if self.y == self.radius {
                    self.next_dir = Direction::Left;
                }
            }
            Direction::Down => {
                self.y -= 1;
                if self.y == -self.radius {
                    self.next_dir = Direction::Right;
                }
            }
        }

        Some(dist)
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn solve(input: usize) -> i32 {
    let mut mem = SpiralMemStressTest::new();

    mem.nth(input - 1).unwrap()
}

fn parse(input: &str) -> usize {
    input
        .parse::<usize>()
        .expect("couldn't parse usize from input")
}

fn main() {
    let input = parse("361527");

    println!("{}", solve(input));
}

#[test]
fn spiralmem_test() {
    let mut mem = SpiralMemStressTest::new();
    assert_eq!(mem.next().unwrap(), 1, "address 1");
    assert_eq!(mem.next().unwrap(), 1, "address 2");
    assert_eq!(mem.next().unwrap(), 2, "address 3");
    assert_eq!(mem.next().unwrap(), 4, "address 4");
    assert_eq!(mem.next().unwrap(), 5, "address 5");
    assert_eq!(mem.next().unwrap(), 10, "address 6");
    assert_eq!(mem.next().unwrap(), 11, "address 7");
    assert_eq!(mem.next().unwrap(), 23, "address 8");
    assert_eq!(mem.next().unwrap(), 25, "address 9");
    assert_eq!(mem.next().unwrap(), 26, "address 10");
    assert_eq!(mem.next().unwrap(), 54, "address 11");
    assert_eq!(mem.next().unwrap(), 57, "address 12");
    assert_eq!(mem.next().unwrap(), 59, "address 13");
    assert_eq!(mem.next().unwrap(), 122, "address 14");
    assert_eq!(mem.next().unwrap(), 133, "address 15");
}
