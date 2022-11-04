////! Advent of Code 2017 Day 7

//// https://adventofcode.com/2017/day/7

//use std::rc::Rc;

//use nom::{
//    bytes::complete::tag,
//    character::complete::{alpha1, space1},
//    combinator::{into, map, opt},
//    multi::separated_list1,
//    sequence::{delimited, preceded, terminated, tuple},
//    IResult,
//};

//#[derive(Debug)]
//struct Program<'a> {
//    name: &'a str,
//    weight: u16,
//    above: Vec<Rc<Program<'a>>>,
//}

//impl<'a> From<&'a str> for Program<'a> {
//    fn from(pline: &'a str) -> Self {
//        fn parse(value: &str) -> IResult<&str, (&str, u16, Option<Vec<Program>>)> {
//            tuple((
//                // name
//                terminated(alpha1, space1),
//                // weight
//                delimited(tag("("), nom::character::complete::u16, tag(")")),
//                opt(preceded(
//                    // arrow ->
//                    tuple((space1, tag("->"), space1)),
//                    // names above
//                    map(separated_list1(tag(", "), alpha1), |names: Vec<&str>| {
//                        names.into_iter().map(Program::from).collect()
//                    }),
//                )),
//            ))(value)
//        }

//        if let Ok((.., (name, weight, Some(above)))) = parse(pline) {
//            return Program {
//                name,
//                weight,
//                above,
//            };
//        }

//        Program {
//            name: "NO_NAME",
//            weight: 0,
//            above: vec![],
//        }
//    }
//}

//#[test]
//fn program_parse_test() {
//    let p = Program::from("a (4) -> b, c, d");

//    assert_eq!(p.name, "a");
//    assert_eq!(p.weight, 4);
//    assert_eq!(p.above, vec!["b", "c", "d"]);
//}

//fn main() {
//    let mut input = parse(include_str!("../input"));
//    if let Some(answer) = solve(input) {
//        println!("{}", answer);
//    } else {
//        println!("shrug");
//    }
//}

//fn parse(input: &str) -> Vec<Program> {
//    input
//        .trim()
//        .lines()
//        .map(|line| Program::from(line))
//        .collect()
//}

///// Calculate number of valid passphrases.
//fn solve(mut programs: Vec<Program>) -> Option<&str> {
//    // program names which could be at the bottom (they have progs above)

//    let mut all_above: Vec<&str> = Vec::new();
//    let mut supporters: Vec<&str> = Vec::new();

//    for prog in programs.iter() {
//        if !prog.above.is_empty() {
//            all_above.extend(prog.above.iter());
//            supporters.push(prog.name);
//        }
//    }

//    for supporter in supporters.iter() {
//        if !all_above.contains(supporter) {
//            return Some(supporter);
//        }
//    }

//    None
//}

//#[test]
//fn simple_test() {
//    let input = "pbga (66)
//xhth (57)
//ebii (61)
//havc (66)
//ktlj (57)
//fwft (72) -> ktlj, cntj, xhth
//qoyq (66)
//padx (45) -> pbga, havc, qoyq
//tknk (41) -> ugml, padx, fwft
//jptl (61)
//ugml (68) -> gyxo, ebii, jptl
//gyxo (61)
//cntj (57)";
//    solve(parse(input));
//}
