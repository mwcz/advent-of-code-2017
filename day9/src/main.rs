//! Advent of Code 2017 Day 9

// https://adventofcode.com/2017/day/9

use std::str::Chars;

fn main() {
    let (part1, part2) = solve(include_str!("../input"));

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn solve(input: &str) -> (u16, u16) {
    let sm = StreamMachine::new(input.chars());
    sm.last().unwrap_or((0, 0))
}

#[derive(Copy, Clone)]
enum State {
    EnjoyingLife,
    ObservingGarbage,
}

struct StreamMachine<'chars> {
    stream: Chars<'chars>,
    state: State,
    score: u16,
    depth: u16,
    garbage: u16,
}

impl StreamMachine<'_> {
    fn new(stream: Chars) -> StreamMachine {
        StreamMachine {
            stream,
            state: State::EnjoyingLife,
            score: 0,
            depth: 0,
            garbage: 0,
        }
    }
}

impl Iterator for StreamMachine<'_> {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(ch) = self.stream.next() {
            // No matter what state we're in, ! cancels the following character, so handle that
            // before handling the current state.
            if ch == '!' {
                self.stream.next(); // eat the next char to "Cancel" it
                return Some((self.score, self.garbage));
            }

            match self.state {
                State::EnjoyingLife => match ch {
                    '{' => {
                        self.depth += 1;
                        self.score += self.depth;
                    }
                    '<' => {
                        self.state = State::ObservingGarbage;
                    }
                    '}' => {
                        self.depth -= 1;
                    }
                    _ => {}
                },
                State::ObservingGarbage => {
                    if ch == '>' {
                        self.state = State::EnjoyingLife;
                    } else {
                        self.garbage += 1;
                    }
                }
            }

            Some((self.score, self.garbage))
        } else {
            None
        }
    }
}

#[test]
fn test_groups() {
    assert_eq!(solve("{}"), (1, 0));
    assert_eq!(solve("{{{}}}"), (6, 0));
    assert_eq!(solve("{{},{}}"), (5, 0));
    assert_eq!(solve("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(solve("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
    assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
    assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
    assert_eq!(solve("<>"), (0, 0));
    assert_eq!(solve("<random characters>"), (0, 17));
    assert_eq!(solve("<<<<>"), (0, 3));
    assert_eq!(solve("<{!>}>"), (0, 2));
    assert_eq!(solve("<!!>"), (0, 0));
    assert_eq!(solve("<!!!>>"), (0, 0));
    assert_eq!(solve(r#"<{o"i!a,<{i<a>"#), (0, 10));
}
