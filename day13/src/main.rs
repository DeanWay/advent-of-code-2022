use std::{
    cmp::Ordering,
    io::{stdin, BufRead},
};

use serde_json::{json, Value};

fn main() {
    let input = stdin().lock();
    let pairs = parse_input(input);
    println!("{:?}", solution_1(&pairs));
    println!("{:?}", solution_2(&pairs));
}

fn parse_input(mut input: impl BufRead) -> Vec<(Value, Value)> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str).unwrap();
    input_str
        .split("\n\n")
        .map(|pair_lines| {
            let (first, second) = pair_lines.split_once('\n').unwrap();
            let first: Value = serde_json::from_str(first).unwrap();
            let second: Value = serde_json::from_str(second).unwrap();
            (first, second)
        })
        .collect()
}

fn solution_1(input: &Vec<(Value, Value)>) -> usize {
    (1..)
        .zip(input.iter())
        .filter(|(_, (left, right))| compare_packets(left, right) == Ordering::Less)
        .map(|(i, _)| i)
        .sum()
}

fn solution_2(input: &Vec<(Value, Value)>) -> usize {
    let mut all_packets = Vec::new();
    for (left, right) in input {
        all_packets.push(left);
        all_packets.push(right);
    }
    let divider_1 = json!([[2]]);
    let divider_2 = json!([[6]]);
    all_packets.push(&divider_1);
    all_packets.push(&divider_2);
    all_packets.sort_by(|a, b| compare_packets(a, b));

    let divider_1_idx = all_packets
        .iter()
        .position(|item| *item == &divider_1)
        .unwrap()
        + 1;
    let divider_2_idx = all_packets
        .iter()
        .position(|item| *item == &divider_2)
        .unwrap()
        + 1;
    divider_1_idx * divider_2_idx
}

fn compare_packets(left: &Value, right: &Value) -> Ordering {
    use Value::*;
    let mut left_iter = left.as_array().unwrap().iter();
    let mut right_iter = right.as_array().unwrap().iter();
    loop {
        let x = left_iter.next();
        let y = right_iter.next();
        if x.is_none() && y.is_some() {
            return Ordering::Less;
        }
        if x.is_some() && y.is_none() {
            return Ordering::Greater;
        }
        let (Some(x), Some(y)) = (x, y) else {
            return Ordering::Equal;
        };
        match (x, y) {
            (Number(x), Number(y)) => {
                let x = x.as_i64();
                let y = y.as_i64();
                if x < y {
                    return Ordering::Less;
                } else if x > y {
                    return Ordering::Greater;
                }
            }
            (Array(_), Array(_)) => {
                let order = compare_packets(x, y);
                if order == Ordering::Equal {
                    continue;
                } else {
                    return order;
                }
            }
            (Array(_), Number(y)) => {
                let order = compare_packets(x, &Array(vec![Number(y.clone())]));
                if order == Ordering::Equal {
                    continue;
                } else {
                    return order;
                }
            }
            (Number(x), Array(_)) => {
                let order = compare_packets(&Array(vec![Number(x.clone())]), y);
                if order == Ordering::Equal {
                    continue;
                } else {
                    return order;
                }
            }
            _ => panic!(),
        };
    }
}

#[test]
fn test_compare_packets_example() {
    let example = include_str!("../example.txt");
    let pairs = parse_input(example.as_bytes());
    assert_eq!(pairs.len(), 8);
    let (left, right) = &pairs[0];
    assert_eq!(compare_packets(left, right), Ordering::Less);
    let (left, right) = &pairs[1];
    assert_eq!(compare_packets(left, right), Ordering::Less);
    let (left, right) = &pairs[2];
    assert_eq!(compare_packets(left, right), Ordering::Greater);
    let (left, right) = &pairs[3];
    assert_eq!(compare_packets(left, right), Ordering::Less);
    let (left, right) = &pairs[4];
    assert_eq!(compare_packets(left, right), Ordering::Greater);
    let (left, right) = &pairs[5];
    assert_eq!(compare_packets(left, right), Ordering::Less);
    let (left, right) = &pairs[6];
    assert_eq!(compare_packets(left, right), Ordering::Greater);
    let (left, right) = &pairs[7];
    assert_eq!(compare_packets(left, right), Ordering::Greater);
}
