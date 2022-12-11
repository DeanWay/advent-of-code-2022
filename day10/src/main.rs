use std::io::{stdin, BufRead};

fn main() {
    let input = stdin().lock();
    let commands = parse_input(input);
    println!("solution 1: {:?}", solution_1(&commands));
    println!("solution 2: ");
    for row in solution_2(&commands) {
        for c in row {
            print!("{}", c);
        }
        println!("")
    }
}

#[derive(Debug, Clone)]
enum Command {
    AddX(i32),
    NoOp,
}

fn parse_input(input: impl BufRead) -> Vec<Command> {
    use Command::*;
    input
        .lines()
        .map(|line| line.unwrap())
        .map(
            |line| match line.split(' ').collect::<Vec<&str>>().as_slice() {
                ["noop"] => NoOp,
                ["addx", x] => AddX(x.parse().unwrap()),
                _ => panic!(),
            },
        )
        .collect()
}

fn solution_1(commands: &Vec<Command>) -> Option<i32> {
    let desired_cycles = vec![20, 60, 100, 140, 180, 220];
    let states = normalize_steps(&steps(commands));
    let mut states = states.iter();
    let signal_strengths: Option<Vec<i32>> = desired_cycles
        .iter()
        .map(|&desired_cycle| {
            states
                .find(|state| state.clock_cycle == desired_cycle)
                .map(|state| state.x_value * desired_cycle as i32)
        })
        .collect();
    signal_strengths.map(|vals| vals.iter().sum())
}

fn solution_2(commands: &Vec<Command>) -> Vec<Vec<char>> {
    let states = normalize_steps(&steps(commands));
    let mut result = Vec::new();
    for row in 0..6 {
        let row_start = row * 40;
        result.push(
            states[row_start..row_start + 40]
                .iter()
                .enumerate()
                .map(|(col, state)| {
                    if (state.x_value - col as i32).abs() <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect(),
        );
    }
    result
}

fn normalize_steps(states: &Vec<State>) -> Vec<State> {
    states
        .windows(2)
        .flat_map(|window| {
            (window[0].clock_cycle..window[1].clock_cycle).map(|clock_cycle| State {
                x_value: window[0].x_value,
                clock_cycle,
            })
        })
        .chain([states.last().unwrap().clone()])
        .collect()
}

fn steps(commands: &Vec<Command>) -> Vec<State> {
    let mut states = Vec::new();
    states.push(State {
        x_value: 1,
        clock_cycle: 1,
    });
    let scan = commands.iter().scan(states[0].clone(), step);
    for state in scan {
        states.push(state);
    }
    states
}

#[derive(Debug, Clone)]
struct State {
    x_value: i32,
    clock_cycle: usize,
}

fn step(state: &mut State, command: &Command) -> Option<State> {
    use Command::*;
    match command {
        NoOp => {
            state.clock_cycle += 1;
        }
        AddX(x) => {
            state.x_value += x;
            state.clock_cycle += 2;
        }
    };
    Some(state.clone())
}
