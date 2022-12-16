use colored::Colorize;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    time::Duration,
};

const START_VAL: i32 = 0;
const END_VAL: i32 = 27;

fn main() {
    let input = stdin().lock();
    let height_map = parse_input(input);
    println!("{:?}", solution_1(&height_map));
}

type HeightMap = Vec<Vec<i32>>;
type Position = (usize, usize);

fn parse_input(input: impl BufRead) -> HeightMap {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'S' => START_VAL,
                    'E' => END_VAL,
                    c @ 'a'..='z' => (c as i32 - 'a' as i32) + 1,
                    x => panic!("invalid height value {}", x),
                })
                .collect()
        })
        .collect()
}

fn print_map(height_map: &HeightMap) {
    for row in height_map.iter() {
        for col in row {
            print!("{:0width$} ", col, width = 2);
        }
        println!()
    }
}

fn print_path(height_map: &HeightMap, path: &Vec<Position>) {
    let height = height_map.len();
    let width = height_map[0].len();
    // std::thread::sleep(Duration::from_millis(5));
    // print!("{}[2J", 27 as char);
    println!();
    for row in 0..height {
        for col in 0..width {
            let current_pos = (row, col);
            let val = format!("{:0desired_len$}", height_map[row][col], desired_len = 2);
            if path.contains(&current_pos) {
                print!("{} ", val.green());
            } else {
                print!("{} ", val);
            }
        }
        println!();
    }
    println!();
}

fn find_in_height_map(height_map: &HeightMap, val: i32) -> Option<Position> {
    let height = height_map.len();
    let width = height_map[0].len();
    for r in 0..height {
        for c in 0..width {
            if height_map[r][c] == val {
                return Some((r, c));
            }
        }
    }
    None
}

fn solution_1(height_map: &HeightMap) -> Option<Vec<usize>> {
    let start_pos = find_in_height_map(height_map, START_VAL)?;
    let end_pos = find_in_height_map(height_map, END_VAL)?;
    let res = find_paths(height_map, &start_pos, &end_pos)
        .iter()
        .map(|path| path.len())
        .collect();
    Some(res)
}

fn find_paths(height_map: &HeightMap, start: &Position, end: &Position) -> Vec<Vec<Position>> {
    let mut queue = Vec::new();
    let mut all_paths = Vec::new();
    let mut shortest_paths: HashMap<Position, usize> = HashMap::new();
    queue.push(vec![start.clone()]);
    while !queue.is_empty() {
        let current_path = queue.pop().unwrap();
        // print_path(height_map, &current_path);
        let current_pos = current_path.last().unwrap();
        for neighbor in adjacent_postions(height_map, &current_pos)
            .iter()
            .filter(|adj| {
                is_connected(height_map, &current_pos, &adj) && !current_path.contains(&adj)
            })
            .cloned()
        {
            if let Some(shortest_path_len) = shortest_paths.get(&neighbor) {
                if current_path.len() < *shortest_path_len {
                    shortest_paths.insert(neighbor.clone(), current_path.len());
                } else {
                    continue;
                }
            } else {
                shortest_paths.insert(neighbor.clone(), current_path.len());
            }
            let mut path = current_path.clone();
            path.push(neighbor);
            if neighbor == *end {
                print_path(height_map, &current_path);
                all_paths.push(path);
            } else {
                queue.push(path)
            }
            queue.sort_by(|a, b| b.len().cmp(&a.len()))
        }
    }
    all_paths
}

fn adjacent_postions(height_map: &HeightMap, pos: &Position) -> Vec<Position> {
    let height = height_map.len() as i32;
    let width = height_map[0].len() as i32;
    let r = pos.0.clone() as i32;
    let c = pos.1.clone() as i32;
    [(r, c + 1), (r + 1, c), (r, c - 1), (r - 1, c)]
        .iter()
        .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < height && *c < width)
        .map(|(r, c)| (*r as usize, *c as usize))
        .collect()
}

fn is_connected(height_map: &HeightMap, a: &Position, b: &Position) -> bool {
    let is_above_or_below = a.0.abs_diff(b.0) == 1;
    let is_to_left_or_right = a.1.abs_diff(b.1) == 1;
    let a_val = &height_map[a.0][a.1];
    let b_val = &height_map[b.0][b.1];
    return (is_above_or_below ^ is_to_left_or_right) && (b_val - a_val) <= 1;
}

#[test]
fn test_adjacent_postions() {
    let height_map = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let vals_at_adjacent_pos = |pos| {
        adjacent_postions(&height_map, pos)
            .iter()
            .map(|(r, c)| height_map[*r][*c])
            .collect::<Vec<_>>()
    };
    assert_eq!(
        adjacent_postions(&height_map, &(0, 0)),
        vec![(0, 1), (1, 0)]
    );
    assert_eq!(vals_at_adjacent_pos(&(0, 0)), vec![2, 4]);

    assert_eq!(
        adjacent_postions(&height_map, &(1, 1)),
        vec![(1, 2), (2, 1), (1, 0), (0, 1)]
    );
    assert_eq!(vals_at_adjacent_pos(&(1, 1)), vec![6, 8, 4, 2]);

    assert_eq!(
        adjacent_postions(&height_map, &(1, 0)),
        vec![(1, 1), (2, 0), (0, 0)]
    );
    assert_eq!(vals_at_adjacent_pos(&(1, 0)), vec![5, 7, 1]);

    assert_eq!(
        adjacent_postions(&height_map, &(2, 2)),
        vec![(2, 1), (1, 2)]
    );
    assert_eq!(vals_at_adjacent_pos(&(2, 2)), vec![8, 6]);
}

#[test]
fn test_is_connected() {
    let input = include_str!("../example.txt");
    let height_map = parse_input(input.as_bytes());
    assert!(is_connected(&height_map, &(0, 0), &(0, 1)));
    assert!(is_connected(&height_map, &(0, 0), &(1, 0)));
    assert_eq!(is_connected(&height_map, &(0, 0), &(1, 1)), false);
}

#[test]
fn test_find_paths() {
    let input = include_str!("../example.txt");
    let height_map = parse_input(input.as_bytes());
    let start_pos = find_in_height_map(&height_map, START_VAL).unwrap();
    let end_pos = find_in_height_map(&height_map, END_VAL).unwrap();
    let res = find_paths(&height_map, &start_pos, &end_pos);
    println!("{:?}", res);
}
