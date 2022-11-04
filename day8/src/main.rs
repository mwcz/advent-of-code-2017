//! Advent of Code 2017 Day 8

// https://adventofcode.com/2017/day/8


use std::cmp::Ordering;
use hashbrown::HashMap;

#[allow(clippy::upper_case_acronyms)]
struct CPU<'a> {
    registers: HashMap<&'a str, Register>,
    highest: i32,
}

impl<'a> CPU<'a> {
    /// Buy a new CPU.
    fn new() -> Self {
        Self {
            registers: HashMap::<_, _>::new(),
            highest: 0,
        }
    }

    /// "Register" a register if it doesn't exist.
    fn register_register(&'_ mut self, name: &'a str) {
        self.registers
            .entry(name)
            .or_insert_with(Register::new);
    }

    /// Update the value of a register.
    fn update_register(&'_ mut self, name: &'a str, amount: i32) {
        self.register_register(name);

        if let Some(reg) = self.registers.get_mut(name) {
            reg.0 += amount;
            self.highest = i32::max(self.highest, reg.0);
        }
    }

    fn eval_condition(&'_ mut self, cond: Condition<'a>) -> bool {
        self.register_register(cond.reg_name);

        let cond_reg = &self.registers[cond.reg_name];

        match cond.rel {
            Rel::LessThan => cond_reg.0 < cond.value,
            Rel::LessThanOrEqual => cond_reg.0 <= cond.value,
            Rel::GreaterThan => cond_reg.0 > cond.value,
            Rel::GreaterThanOrEqual => cond_reg.0 >= cond.value,
            Rel::Equal => cond_reg.0 == cond.value,
            Rel::NotEqual => cond_reg.0 != cond.value,
        }
    }

    fn exec(&'_ mut self, inst: Instruction<'a>) {
        if self.eval_condition(inst.cond) {
            self.update_register(inst.reg_name, inst.add_amount);
        }
    }

    /// Find the highest among all registers.
    fn max(&'a self) -> Option<i32> {
        self.registers
            .iter()
            .max_by(|&(_, reg1), &(_, reg2)| reg1.partial_cmp(reg2).unwrap())
            .map(|max| max.1.0)
    }
}

struct Register(i32);

impl Register {
    fn new() -> Self {
        Self(0)
    }
}

impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Register {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

struct Instruction<'a> {
    reg_name: &'a str,
    add_amount: i32,
    cond: Condition<'a>,
}

struct Condition<'a> {
    reg_name: &'a str,
    rel: Rel,
    value: i32,
}

enum Rel {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[test]
fn test_register_operations() {
    let mut cpu = CPU::new();

    assert!(cpu.eval_condition(Condition {
        reg_name: "foo",
        rel: Rel::Equal,
        value: 0,
    }));

    cpu.update_register("foo", 42);

    assert!(cpu.eval_condition(Condition {
        reg_name: "foo",
        rel: Rel::Equal,
        value: 42,
    }));

    cpu.update_register("foo", 13);

    assert!(cpu.eval_condition(Condition {
        reg_name: "foo",
        rel: Rel::Equal,
        value: 55,
    }));

    cpu.update_register("foo", -58);

    assert!(cpu.eval_condition(Condition {
        reg_name: "foo",
        rel: Rel::Equal,
        value: -3,
    }));
}

#[test]
fn test_cpu_instructions() {
    let mut cp = CPU::new();

    cpu.exec(Instruction {
        reg_name: "foo",
        add_amount: 4,
        cond: Condition {
            reg_name: "bar",
            rel: Rel::GreaterThan,
            value: -1,
        },
    });

    assert!(cpu.eval_condition(Condition {
        reg_name: "foo",
        rel: Rel::Equal,
        value: 4,
    }));
}

fn main() {
    let input = parse(include_str!("../input"));

    let (part1, part2) = solve(input);

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut sp = line.split_whitespace();
            let reg_name = sp.next().unwrap();
            let op = match sp.next().unwrap() {
                "inc" => sp.next().unwrap().parse::<i32>().unwrap(),
                "dec" => -sp.next().unwrap().parse::<i32>().unwrap(),
                _ => panic!("invalid operation"),
            };

            // consume "if"
            sp.next().unwrap();

            let cond = Condition {
                reg_name: sp.next().unwrap(),
                rel: match sp.next().unwrap() {
                    "<" => Rel::LessThan,
                    "<=" => Rel::LessThanOrEqual,
                    "==" => Rel::Equal,
                    "!=" => Rel::NotEqual,
                    ">" => Rel::GreaterThan,
                    ">=" => Rel::GreaterThanOrEqual,
                    _ => panic!("invalid condition"),
                },
                value: sp.next().unwrap().parse::<i32>().unwrap(),
            };
            Instruction { reg_name, add_amount: op, cond }
        })
        .collect()
}

/// Find the value of the highest-valued register after executing the input instructions.
fn solve(instructions: Vec<Instruction>) -> (i32, i32) {
    let mut cpu = CPU::new();

    for inst in instructions {
        cpu.exec(inst);
    }

    (cpu.max().unwrap(), cpu.highest)
}
