use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let monkey_business = include_str!("day11.txt").trim().replace("\r", "");

    const NUM_ROUNDS: i32 = 20;
    let mut monkeys: Vec<_> = monkey_business.split("\n\n").map(Monkey::new).collect();

    //=============================================================================================
    // Simulate the monkey game
    //=============================================================================================
    for _ in 0..NUM_ROUNDS {
        for i in 0..monkeys.len() { // Not using iterator on monkeys so we can modify other monkeys
            while let Some((item, dest)) = monkeys[i].inspect_one() {
                monkeys[dest].items.push_back(item);
            }
        }
    }

    //=============================================================================================
    // Part 1: Calculate the level of monkey business
    //=============================================================================================
    monkeys.sort_by_key(|m| m.inspect_count);
    let monkey_business_level: u32 = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspect_count)
        .product();
    println!("Part 1: {monkey_business_level}");
}

type Item = i32;

#[derive(Debug)]
struct Monkey {
    #[allow(unused)]
    id: u32,
    items: VecDeque<Item>,
    operation: Operation,
    test_value: i32,
    if_true_dest: usize,
    if_false_dest: usize,
    inspect_count: u32,
}

impl Monkey {
    fn new(notes: &str) -> Monkey {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(concat!(
                r"^Monkey (?P<id>\d+):\n",
                r"\s+Starting items: (?P<items>\d+(?:, \d+)*)\n",
                r"\s+Operation: new = (?P<operand1>old|\d+) (?P<operator>\*|\+) (?P<operand2>old|\d+)\n",
                r"\s+Test: divisible by (?P<test_value>\d+)\n",
                r"\s+If true: throw to monkey (?P<if_true_dest>\d+)\n",
                r"\s+If false: throw to monkey (?P<if_false_dest>\d+)$",
            )).unwrap();
        }

        let captures = REGEX.captures(notes).unwrap();

        let operand1 = captures["operand1"].parse::<Operand>().unwrap();
        let operand2 = captures["operand2"].parse::<Operand>().unwrap();

        Monkey {
            id: captures["id"].parse().unwrap(),
            items: captures["items"]
                .split(",")
                .map(str::trim)
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
            operation: match &captures["operator"] {
                "+" => Operation::Add(operand1, operand2),
                "*" => Operation::Mul(operand1, operand2),
                x => panic!("invalid operation '{}'", x),
            },
            test_value: captures["test_value"].parse().unwrap(),
            if_true_dest: captures["if_true_dest"].parse().unwrap(),
            if_false_dest: captures["if_false_dest"].parse().unwrap(),
            inspect_count: 0,
        }
    }

    /// Return value is pair of [item thrown, destination monkey index] -- or none if no items left to inspect
    fn inspect_one(&mut self) -> Option<(i32, usize)> {
        let mut item = self.items.pop_front()?;

        // Apply worry operation
        item = self.operation.calculate(item);

        // Calm down
        item /= 3;

        // Log this inspection
        self.inspect_count += 1;

        // Determine who to throw it to
        if item % self.test_value == 0 {
            Some((item, self.if_true_dest))
        } else {
            Some((item, self.if_false_dest))
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    fn calculate(&self, old: i32) -> i32 {
        match self {
            Operation::Add(a, b) => a.get_value(old) + b.get_value(old),
            Operation::Mul(a, b) => a.get_value(old) * b.get_value(old),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(i32),
}

impl Operand {
    fn get_value(&self, old: i32) -> i32 {
        match self {
            Operand::Old => old,
            Operand::Num(x) => *x,
        }
    }
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Operand::Old)
        } else {
            let num: i32 = s.parse()?;
            Ok(Operand::Num(num))
        }
    }
}
