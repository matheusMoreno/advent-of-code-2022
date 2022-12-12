use std::collections::VecDeque;
use std::fs;

use regex::{Regex, Captures};

const INPUT_FILE_PATH: &str = "input";

const NUMBER_OF_ROUNDS: usize = 10_000;  // = 20 for Part 1


fn main() {
    let input: String = read_input_file(INPUT_FILE_PATH);

    let mut monkeys: Vec<Monkey> = generate_monkey_vector(&input);
    let divisible_by_all: u128 = monkeys.iter().map(|x| x.divisible_by).product();
    execute_rounds(&mut monkeys, divisible_by_all);
    let monkey_business: u128 = compute_monkey_business(&mut monkeys);

    println!("Number of monkey business: {monkey_business}");
}


fn read_input_file(file_path: &str) -> String {
    let file_contents: String = fs::read_to_string(file_path)
        .expect("Unable to read the file...");

    file_contents
}


#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: (char, String),
    divisible_by: u128,
    monkey_if_true: usize,
    monkey_if_false: usize,
    items_inspected: u128,
}


impl Monkey {
    pub fn new(spec: &str) -> Monkey {
        let monkey_spec: Captures = Regex::new(concat!(
            r"Monkey \d+:",
            r"\s*Starting items: (?P<starting_items>.+)",
            r"\s*Operation: new = old (?P<operator>.) (?P<value>.+)",
            r"\s*Test: divisible by (?P<divisible_by>\d+)",
            r"\s*If true: throw to monkey (?P<monkey_if_true>\d+)",
            r"\s*If false: throw to monkey (?P<monkey_if_false>\d+)",
        )).unwrap().captures_iter(spec).next().unwrap();

        Monkey {
            items: VecDeque::from_iter(
                monkey_spec["starting_items"].split(",").map(
                    |x| x.trim().parse::<u128>().unwrap()
                )
            ),
            operation: (
                monkey_spec["operator"].chars().next().unwrap(),
                String::from(&monkey_spec["value"])
            ),
            divisible_by: monkey_spec["divisible_by"].parse().unwrap(),
            monkey_if_true: monkey_spec["monkey_if_true"].parse().unwrap(),
            monkey_if_false: monkey_spec["monkey_if_false"].parse().unwrap(),
            items_inspected: 0
        }
    }

    pub fn take_turn(&mut self, decrease_factor: u128) -> (VecDeque<u128>, VecDeque<u128>) {
        let mut items_to_true: VecDeque<u128> = VecDeque::new();
        let mut items_to_false: VecDeque<u128> = VecDeque::new();

        while !self.items.is_empty() {
            let mut worry_level: u128 = self.items.pop_front().unwrap();

            worry_level = self.apply_operation(worry_level);

            // Line below for Part 1
            // worry_level = (worry_level as f64 / 3.0).floor() as u128;
            worry_level = worry_level % decrease_factor;

            match self.test_new_worry_level(worry_level) {
                true => items_to_true.push_back(worry_level),
                false => items_to_false.push_back(worry_level),
            };

            self.items_inspected += 1;
        }

        (items_to_true, items_to_false)
    }

    pub fn apply_operation(&self, value: u128) -> u128 {
        match (self.operation.0, self.operation.1.as_str()) {
            ('+', "old") => value + value,
            ('*', "old") => value * value,
            ('+', to_add) => value + to_add.parse::<u128>().unwrap(),
            ('*', to_mul) => value * to_mul.parse::<u128>().unwrap(),
            _ => todo!(),
        }
    }

    pub fn test_new_worry_level(&self, worry_level: u128) -> bool {
        if worry_level % self.divisible_by == 0 {
            return true;
        }

        false
    }
}


fn generate_monkey_vector(input: &str) -> Vec<Monkey> {
    Vec::from_iter(
        input.split("\n\n").map(|spec| Monkey::new(spec))
    )
}


fn execute_rounds(monkeys: &mut Vec<Monkey>, decrease_factor: u128) -> () {
    for _ in 0..NUMBER_OF_ROUNDS {
        for i in 0..monkeys.len() {
            let (mut to_true, mut to_false) = monkeys[i].take_turn(decrease_factor);
            let monkey_if_true: usize = monkeys[i].monkey_if_true;
            let monkey_if_false: usize = monkeys[i].monkey_if_false;
            monkeys[monkey_if_true].items.append(&mut to_true);
            monkeys[monkey_if_false].items.append(&mut to_false);
        }
    }
}


fn compute_monkey_business(monkeys: &mut Vec<Monkey>) -> u128 {
    monkeys.sort_by_key(|m| m.items_inspected);
    monkeys[monkeys.len() - 1].items_inspected * monkeys[monkeys.len() - 2].items_inspected
}
