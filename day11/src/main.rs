mod input;
use num::integer::lcm;
use std::{env::args, iter::successors};

use crate::input::{example, input, Monkey};

fn main() {
    let args: Vec<String> = args().collect();
    let monkeys = match args[1].as_str() {
        "example" => example(),
        "input" => input(),
        _ => panic!("invalid input"),
    };

    println!("{:?}", solution_1(&monkeys));
    println!("{:?}", solution_2(&monkeys));
}

fn solution_1(monkeys: &Vec<Monkey>) -> u64 {
    let mut rounds = successors(Some(monkeys.clone()), |prev_round| {
        Some(run_round(prev_round, |worry| worry / 3))
    });
    let mut monkey_inspections: Vec<_> = rounds
        .nth(20)
        .unwrap()
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();
    monkey_inspections.sort_by(|a, b| b.cmp(a));
    monkey_inspections[0] * monkey_inspections[1]
}

fn solution_2(monkeys: &Vec<Monkey>) -> u64 {
    let lcm_divisor = monkeys
        .iter()
        .fold(1, |divisor, monkey| lcm(divisor, monkey.test_divisible_by));
    let mut rounds = successors(Some(monkeys.clone()), move |prev_round| {
        Some(run_round(prev_round, |worry| worry % lcm_divisor))
    });
    let mut monkey_inspections: Vec<_> = rounds
        .nth(10000)
        .unwrap()
        .iter()
        .map(|monkey| monkey.inspections)
        .collect();
    monkey_inspections.sort_by(|a, b| b.cmp(a));
    monkey_inspections[0] * monkey_inspections[1]
}

fn run_round(monkeys: &Vec<Monkey>, worry_reduction: impl Fn(u64) -> u64) -> Vec<Monkey> {
    let mut new_monkeys = monkeys.clone();
    for monkey_num in 0..new_monkeys.len() {
        let items = std::mem::replace(&mut new_monkeys[monkey_num].items, Vec::new());
        new_monkeys[monkey_num].inspections += items.len() as u64;
        for item in items {
            let operation = &new_monkeys[monkey_num].operation;
            let new = operation(item);
            let new = worry_reduction(new);
            let monkey_to_throw_to = if new % &new_monkeys[monkey_num].test_divisible_by == 0 {
                new_monkeys[monkey_num].if_true_throw_to
            } else {
                new_monkeys[monkey_num].if_false_throw_to
            };
            new_monkeys[monkey_to_throw_to].items.push(new)
        }
    }
    new_monkeys
}
