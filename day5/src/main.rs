use std::{collections::VecDeque, io::stdin};

use anyhow::{Error, Result};
use regex::Regex;

fn main() -> Result<()> {
    let input = parse_input()?;
    println!("{:?}", solution_1(&input));
    println!("{:?}", solution_2(&input));
    Ok(())
}

#[derive(Debug)]
struct Input {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input() -> Result<Input> {
    let mut lines = stdin().lines().map(|line| line.unwrap());
    let stack_lines: Vec<String> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    let move_lines: Vec<String> = lines.collect();
    let stacks = parse_stacks(stack_lines)?;
    let moves = parse_moves(move_lines)?;
    Ok(Input { stacks, moves })
}

fn parse_stacks(lines: Vec<String>) -> Result<Vec<VecDeque<char>>> {
    let last = lines
        .last()
        .ok_or(Error::msg("stack lines cannot be emtpy"))?;

    let mut result = Vec::new();
    for (i, char) in last.chars().enumerate() {
        if !char.is_whitespace() {
            let mut stack = VecDeque::new();
            for line in lines.iter().rev().skip(1) {
                let char = line.chars().nth(i).unwrap();
                if !char.is_whitespace() {
                    stack.push_front(char)
                }
            }
            result.push(stack);
        }
    }
    Ok(result)
}

fn parse_moves(lines: Vec<String>) -> Result<Vec<Move>> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    lines
        .iter()
        .map(|line| {
            let captures = re
                .captures(&line)
                .ok_or(Error::msg(format!("invalid move line: {}", line)))?;
            Ok(Move {
                amount: captures[1].parse()?,
                from: (captures[2].parse::<usize>()? - 1),
                to: (captures[3].parse::<usize>()? - 1),
            })
        })
        .collect()
}

fn solution_1(input: &Input) -> String {
    let stacks_after_moves = run_moves_solution_1(input);
    stacks_after_moves
        .iter()
        .map(|stack| stack.front().unwrap_or(&' '))
        .collect()
}

fn run_moves_solution_1(input: &Input) -> Vec<VecDeque<char>> {
    let mut result = input.stacks.clone();
    for current_move in input.moves.iter() {
        for _ in 0..current_move.amount {
            if let Some(c) = result[current_move.from].pop_front() {
                result[current_move.to].push_front(c)
            }
        }
    }
    result
}

fn solution_2(input: &Input) -> String {
    let stacks_after_moves = run_moves_solution_2(input);
    stacks_after_moves
        .iter()
        .map(|stack| stack.front().unwrap_or(&' '))
        .collect()
}

fn run_moves_solution_2(input: &Input) -> Vec<VecDeque<char>> {
    let mut result = input.stacks.clone();
    for current_move in input.moves.iter() {
        let mut intermediate_stack = VecDeque::with_capacity(current_move.amount);
        for _ in 0..current_move.amount {
            if let Some(c) = result[current_move.from].pop_front() {
                intermediate_stack.push_front(c);
            }
        }
        for c in intermediate_stack {
            result[current_move.to].push_front(c);
        }
    }
    result
}
