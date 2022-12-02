use std::io::stdin;
use GameResult::*;
use RPS::*;

fn main() -> Result<(), String> {
    let input = parse_input()?;
    println!("{:?}", solution_1(&input));
    println!("{:?}", solution_2(&input));
    Ok(())
}

fn parse_input() -> Result<Vec<(String, String)>, String> {
    stdin()
        .lines()
        .map(|line| parse_line(line.map_err(|e| e.to_string())?.as_str()))
        .collect()
}

fn parse_line(line: &str) -> Result<(String, String), String> {
    let mut split = line.split(" ");
    let first = split.next().ok_or("invalid line")?.to_string();
    let snd = split.next().ok_or("invalid line")?.to_string();
    Ok((first, snd))
}

fn solution_1(input: &Vec<(String, String)>) -> i32 {
    input
        .iter()
        .map(solution_1_interpret_pair)
        .map(|(opp, player)| round_value(&(player, opp)))
        .sum()
}

fn solution_1_interpret_pair(pair: &(String, String)) -> (RPS, RPS) {
    let (opp_str, player_str) = pair;
    let opp = match opp_str.as_str() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!(),
    };
    let player = match player_str.as_str() {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!(),
    };
    (opp, player)
}

fn solution_2(input: &Vec<(String, String)>) -> i32 {
    input
        .iter()
        .map(solution_2_interpret_pair)
        .map(|(opp, player)| round_value(&(player, opp)))
        .sum()
}

fn solution_2_interpret_pair(pair: &(String, String)) -> (RPS, RPS) {
    let (opp_str, player_str) = pair;
    let opp = match opp_str.as_str() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!(),
    };
    let desired_result = match player_str.as_str() {
        "X" => Loss,
        "Y" => Tie,
        "Z" => Win,
        _ => panic!(),
    };
    let player = match desired_result {
        Tie => opp,
        Loss => match opp {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Win => match opp {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
    };
    (opp, player)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum GameResult {
    Win,
    Loss,
    Tie,
}

impl GameResult {
    fn value(&self) -> i32 {
        match self {
            Win => 6,
            Tie => 3,
            Loss => 0,
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn round_result(round: &(RPS, RPS)) -> GameResult {
    match round {
        (x, y) if x == y => Tie,
        (Paper, Rock) => Win,
        (Rock, Scissors) => Win,
        (Scissors, Paper) => Win,
        _ => Loss,
    }
}

fn round_value(round: &(RPS, RPS)) -> i32 {
    let result = round_result(round);
    let (player, _) = round;
    return result.value() + player.value();
}
