//! Advent of Code 2017 Day 3

// --- Day 3: Spiral Memory ---
//
// --- Part One ---
// You come across an experimental new kind of memory stored on an infinite two-dimensional grid.
//
// Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and
// then counting up while spiraling outward. For example, the first few squares are allocated like
// this:
//
//   17  16  15  14  13
//   18   5   4   3  12
//   19   6   1   2  11
//   20   7   8   9  10
//   21  22  23---> ...
//
// While this is very space-efficient (no squares are skipped), requested data must be carried back
// to square 1 (the location of the only access port for this memory system) by programs that can
// only move up, down, left, or right. They always take the shortest path: the Manhattan Distance
// between the location of the data and square 1.
//
// For example:
//
// - Data from square 1 is carried 0 steps, since it's at the access port.
// - Data from square 12 is carried 3 steps, such as: down, left, left.
// - Data from square 23 is carried only 2 steps: up twice.
// - Data from square 1024 must be carried 31 steps.
//
// How many steps are required to carry the data from the square identified in your puzzle input
// all the way to the access port?
//
// Your puzzle input is 361527.

struct SpiralMem {
    /// The current X coordinate.
    x: i32,
    /// The current Y coordinate.
    y: i32,
    /// Half the (width or height) of the current square spiral arm, floored.
    radius: i32,
    /// The travel direction for the next iteration.
    next_dir: Direction,
}

impl SpiralMem {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            radius: 0,
            next_dir: Direction::Right,
        }
    }
}

impl Iterator for SpiralMem {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let dist = self.x.abs() + self.y.abs();

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
                self.y += 1;

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
    let mut mem = SpiralMem::new();

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
    let mut mem = SpiralMem::new();
    // assert_eq!(mem.next().unwrap(), 0, "address 1");
    // assert_eq!(mem.next().unwrap(), 1, "address 2");
    // assert_eq!(mem.next().unwrap(), 2, "address 3");
    // assert_eq!(mem.next().unwrap(), 1, "address 4");
    // assert_eq!(mem.next().unwrap(), 2, "address 5");
    // assert_eq!(mem.next().unwrap(), 1, "address 6");
    // assert_eq!(mem.next().unwrap(), 2, "address 7");
    // assert_eq!(mem.next().unwrap(), 1, "address 8");
    // assert_eq!(mem.next().unwrap(), 2, "address 9");
    // assert_eq!(mem.next().unwrap(), 3, "address 10");
    // assert_eq!(mem.next().unwrap(), 2, "address 11");
    // assert_eq!(mem.next().unwrap(), 3, "address 12");
    // assert_eq!(mem.next().unwrap(), 4, "address 13");
    // assert_eq!(mem.next().unwrap(), 3, "address 14");
    // assert_eq!(mem.next().unwrap(), 2, "address 15");
    // assert_eq!(mem.next().unwrap(), 3, "address 16");
    // assert_eq!(mem.next().unwrap(), 4, "address 17");
    // assert_eq!(mem.next().unwrap(), 3, "address 18");
    // assert_eq!(mem.next().unwrap(), 2, "address 19");
    // assert_eq!(mem.next().unwrap(), 3, "address 20");
    // assert_eq!(mem.next().unwrap(), 4, "address 21");
    // assert_eq!(mem.next().unwrap(), 3, "address 22");
    // assert_eq!(mem.next().unwrap(), 2, "address 23");
    // assert_eq!(mem.next().unwrap(), 3, "address 24");
    // assert_eq!(mem.next().unwrap(), 4, "address 25");
    // assert_eq!(mem.next().unwrap(), 5, "address 26");
    let x = 13;
    assert_eq!(mem.nth(x).unwrap(), 4, "address 13");
    foo(13);
}

fn foo(x: u8) {
    println!("{x}");
}
