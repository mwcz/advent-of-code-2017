//! A solution to day 3 year .
//! https://adventofcode.com//day/3

type Model = usize;
type Answer = i32;

pub fn parse(input: String) -> Model {
    input
        .parse::<usize>()
        .expect("couldn't parse usize from input")
}

pub fn part1(input: Model) -> Answer {
    let mut mem = SpiralMem::new();

    mem.nth(input - 1).unwrap()
}

pub fn part2(input: Model) -> Answer {
    let mut mem = SpiralMemStressTest::new();

    mem.nth(input - 1).unwrap()
}

struct SpiralMem {
    /// The current X coordinate.
    x: i32,
    /// The current Y coordinate.
    y: i32,
    /// Half the (width or height) of the current square spiral arm, floored.
    radius: i32,
    /// The travel direction for the next iteration.
    next_dir: Direction,
    /// Part 1 algorithm or part 2?
    part: Part,
}

enum Part {
    P1,
    P2,
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
        // apply the appropriate algorithm for each part
        match self.part {
            Part::P1 => {
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
            Part::P2 => {
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
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d3");
    const EXAMPLE: &str = include_str!("../examples/d3");

    #[test]
    fn d3p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 31);
    }

    #[test]
    fn d3p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 326);
    }

    #[test]
    fn d3p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())),);
    }

    #[test]
    fn d3p2_input_test() {
        assert_eq!(
            part2(parse(INPUT.to_string())),
            "put part 2 final answer here"
        );
    }
}
