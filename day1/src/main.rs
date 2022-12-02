use std::io::stdin;

use itertools::Itertools;
use std::collections::BinaryHeap;

fn main() {
    let input = parse_input();
    println!("solution 1: {:?}", solution_1(&input));
    println!("solution 2: {:?}", solution_2(&input));
}

fn parse_input() -> Vec<Vec<i32>> {
    let lines_parsed = stdin().lines().map(|line| line.unwrap().parse::<i32>());

    let mut groups = Vec::new();
    for (key, group) in &lines_parsed.group_by(|x| x.is_ok()) {
        if key {
            groups.push(group.map(|x| x.unwrap()).collect_vec());
        }
    }
    groups
}

fn solution_1(input: &Vec<Vec<i32>>) -> Option<i32> {
    input.iter().map(|group| group.iter().sum()).max()
}

fn solution_2(input: &Vec<Vec<i32>>) -> Option<i32> {
    let group_totals = input.iter().map(|group| group.iter().sum::<i32>());
    let mut heap = BinaryHeap::from_iter(group_totals);
    let mut sum = 0;
    for _ in 0..3 {
        match heap.pop() {
            Some(val) => sum += val,
            None => return None,
        };
    }
    Some(sum)
}
