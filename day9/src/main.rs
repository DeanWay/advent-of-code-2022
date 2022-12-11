use std::{
    collections::HashSet,
    io::{stdin, BufRead},
    iter::successors,
};
fn main() {
    let input = stdin().lock();
    let moves = parse_input(input);
    println!("{:?}", solution_1(&moves));
    println!("{:?}", solution_2(&moves));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: usize,
}

type Position = (i32, i32);

type Rope = Vec<Position>;

fn parse_input(input: impl BufRead) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            use Direction::*;
            let (direction, amount) = line.split_once(' ').unwrap();
            let amount: usize = amount.parse().unwrap();
            let direction = match direction {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!(),
            };
            Move { direction, amount }
        })
        .collect()
}

fn solution_1(moves: &Vec<Move>) -> usize {
    count_tail_positions(moves, &vec![(0, 0); 2])
}

fn solution_2(moves: &Vec<Move>) -> usize {
    count_tail_positions(moves, &vec![(0, 0); 10])
}

fn count_tail_positions(moves: &Vec<Move>, rope: &Rope) -> usize {
    let mut tail_positions = HashSet::new();
    steps(moves, rope).iter().for_each(|step| {
        tail_positions.insert(step.last().unwrap().clone());
    });
    tail_positions.len()
}

fn steps(moves: &Vec<Move>, rope: &Rope) -> Vec<Rope> {
    let mut directions = moves.iter().flat_map(unit_moves);
    successors(Some(rope.clone()), |current_rope| {
        if let Some(current_direction) = directions.next() {
            let next_positions = step(&current_direction, current_rope);
            Some(next_positions)
        } else {
            None
        }
    })
    .collect()
}

fn unit_moves(m: &Move) -> Vec<Direction> {
    (0..m.amount).map(|_| m.direction).collect()
}

fn step(current_move: &Direction, rope: &Rope) -> Rope {
    use Direction::*;
    let (head_row, head_col) = rope.first().unwrap().clone();
    let next_head = match current_move {
        Up => (head_row + 1, head_col),
        Down => (head_row - 1, head_col),
        Right => (head_row, head_col + 1),
        Left => (head_row, head_col - 1),
    };
    let mut next_rope = rope.clone();
    next_rope[0] = next_head;
    for i in 0..next_rope.len() - 1 {
        let fst = next_rope[i];
        let snd = next_rope[i + 1];
        if !are_touching(&fst, &snd) {
            next_rope[i + 1] = rope[i];
        }
    }
    next_rope
}

fn are_touching(a: &Position, b: &Position) -> bool {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    (a_x - b_x).abs() <= 1 && (a_y - b_y).abs() <= 1
}

#[test]
fn test_solution_2_example_steps() {
    let example_txt = include_str!("../example.txt");
    let moves = parse_input(example_txt.as_bytes());
    assert_eq!(
        steps(&moves, &vec![(0, 0); 10]),
        vec![
            vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (0, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (4, 4),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
                (0, 0)
            ],
            vec![
                (4, 2),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0)
            ],
            vec![
                (4, 1),
                (4, 2),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (3, 1),
                (4, 2),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (3, 3),
                (4, 2),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (3, 4),
                (3, 3),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (3, 5),
                (3, 4),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 5),
                (3, 4),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 4),
                (3, 4),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 3),
                (3, 4),
                (4, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 2),
                (2, 3),
                (3, 4),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 1),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ],
            vec![
                (2, 2),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (1, 4),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0)
            ]
        ]
    );
}
