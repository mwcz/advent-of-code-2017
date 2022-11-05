//! Advent of Code 2017 Day 8

// https://adventofcode.com/2017/day/8

use heapless::{Entry, FnvIndexMap};
use std::cmp::Ordering;

fn main() {
    let input = parse(include_str!("../input"));

    let (part1, part2) = solve(input);

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn parse(input: &str) -> Vec<AddInstruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut sp = line.split_whitespace();

            // register name
            let reg_name = sp.next().unwrap();

            // inc/dec amount
            let amount = match sp.next().unwrap() {
                "inc" => sp.next().unwrap().parse::<i32>().unwrap(),
                "dec" => -sp.next().unwrap().parse::<i32>().unwrap(),
                _ => panic!("unrecognized opcode"),
            };

            // consume "if"
            sp.next().unwrap();

            // condition expression
            let cond = Condition {
                reg_name: sp.next().unwrap(),
                operator: sp
                    .next()
                    .unwrap()
                    .try_into()
                    .expect("unrecognized conditional operator"),
                value: sp.next().unwrap().parse::<i32>().unwrap(),
            };

            AddInstruction {
                reg_name,
                amount,
                cond,
            }
        })
        .collect()
}

/// Find the value of the highest-valued register after executing the input instructions.
fn solve(instructions: Vec<AddInstruction>) -> (i32, i32) {
    let mut cpu = CPU::new();

    for inst in instructions {
        cpu.exec(inst);
    }

    (cpu.max().unwrap(), cpu.highest)
}

const MAX_REGISTER_COUNT: usize = 32;

#[allow(clippy::upper_case_acronyms)]
struct CPU<'a> {
    registers: FnvIndexMap<&'a str, Register, MAX_REGISTER_COUNT>,
    highest: i32,
}

impl<'a> CPU<'a> {
    /// Buy a new CPU.
    fn new() -> Self {
        Self {
            registers: FnvIndexMap::<_, _, MAX_REGISTER_COUNT>::new(),
            highest: 0,
        }
    }

    /// "Register" a register if it doesn't exist.
    fn register_register(&'_ mut self, name: &'a str) {
        if let Entry::Vacant(v) = self.registers.entry(name) {
            v.insert(Register::new())
                .expect("max register count exceeded");
        }
    }

    /// Update the value of a register.
    fn update_register(&'_ mut self, name: &'a str, amount: i32) {
        self.register_register(name);

        if let Some(reg) = self.registers.get_mut(name) {
            reg.0 += amount;
            self.highest = i32::max(self.highest, reg.0);
        }
    }

    /// Evalutate whether a condition is true or false.
    fn eval_condition(&'_ mut self, cond: Condition<'a>) -> bool {
        self.register_register(cond.reg_name);

        let cond_reg = &self.registers[cond.reg_name];

        match cond.operator {
            Operator::LessThan => cond_reg.0 < cond.value,
            Operator::LessThanOrEqual => cond_reg.0 <= cond.value,
            Operator::GreaterThan => cond_reg.0 > cond.value,
            Operator::GreaterThanOrEqual => cond_reg.0 >= cond.value,
            Operator::Equal => cond_reg.0 == cond.value,
            Operator::NotEqual => cond_reg.0 != cond.value,
        }
    }

    /// Execute an instruction on this CPU.
    fn exec(&'_ mut self, inst: AddInstruction<'a>) {
        if self.eval_condition(inst.cond) {
            self.update_register(inst.reg_name, inst.amount);
        }
    }

    /// Find the highest among all registers.
    fn max(&'a self) -> Option<i32> {
        self.registers
            .iter()
            .max_by(|&(_, reg1), &(_, reg2)| reg1.partial_cmp(reg2).unwrap())
            .map(|max| max.1 .0)
    }
}

#[derive(Debug)]
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

struct AddInstruction<'a> {
    reg_name: &'a str,
    amount: i32,
    cond: Condition<'a>,
}

struct Condition<'a> {
    reg_name: &'a str,
    operator: Operator,
    value: i32,
}

// I wanted to use std::cmp::Ordering but it only includes Less, Equal, and Greater.
enum Operator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

impl TryFrom<&str> for Operator {
    type Error = ();

    #[rustfmt::skip]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<"  => Ok(Self::LessThan),
            "<=" => Ok(Self::LessThanOrEqual),
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            ">"  => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterThanOrEqual),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_operations() {
        let mut cpu = CPU::new();

        assert!(cpu.eval_condition(Condition {
            reg_name: "foo",
            operator: Operator::Equal,
            value: 0,
        }));

        cpu.update_register("foo", 42);

        assert!(cpu.eval_condition(Condition {
            reg_name: "foo",
            operator: Operator::Equal,
            value: 42,
        }));

        cpu.update_register("foo", 13);

        assert!(cpu.eval_condition(Condition {
            reg_name: "foo",
            operator: Operator::Equal,
            value: 55,
        }));

        cpu.update_register("foo", -58);

        assert!(cpu.eval_condition(Condition {
            reg_name: "foo",
            operator: Operator::Equal,
            value: -3,
        }));
    }

    #[test]
    fn test_cpu_instructions() {
        let mut cpu = CPU::new();

        cpu.exec(AddInstruction {
            reg_name: "foo",
            amount: 4,
            cond: Condition {
                reg_name: "bar",
                operator: Operator::GreaterThan,
                value: -1,
            },
        });

        assert!(cpu.eval_condition(Condition {
            reg_name: "foo",
            operator: Operator::Equal,
            value: 4,
        }));
    }
}
