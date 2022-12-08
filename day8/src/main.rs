use std::io::{stdin, BufRead};
use std::iter::repeat;

fn main() {
    let input = stdin().lock();
    let grid = parse_input(input);
    println!("{:?}", solution_1(&grid));
    println!("{:?}", solution_2(&grid));
}
type Grid = Vec<Vec<i32>>;

fn parse_input(input: impl BufRead) -> Grid {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solution_1(grid: &Grid) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;
    for r in 0..height {
        for c in 0..width {
            if is_visible((r, c), grid) {
                count += 1;
            }
        }
    }
    count
}

fn solution_2(grid: &Grid) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut positions = Vec::new();
    for r in 0..height {
        for c in 0..width {
            positions.push((r, c))
        }
    }
    positions
        .iter()
        .map(|pos| scenic_score(*pos, grid))
        .max()
        .unwrap()
}

fn scenic_score(pos: (usize, usize), grid: &Grid) -> usize {
    let (r, c) = pos;
    let value = grid[r][c];
    paths_to_edges(pos, grid)
        .iter()
        .map(|path| {
            path.iter()
                .enumerate()
                .find(|(_, other_val)| **other_val >= value)
                .map(|(i, _)| i + 1)
                .unwrap_or(path.len())
        })
        .fold(1, |x, y| x * y)
}

fn is_visible(pos: (usize, usize), grid: &Grid) -> bool {
    let (r, c) = pos;
    let value = grid[r][c];
    paths_to_edges(pos, grid)
        .iter()
        .map(|path| path.iter().all(|other_val| *other_val < value))
        .any(|visible_sightline| visible_sightline)
}

fn paths_to_edges(pos: (usize, usize), grid: &Grid) -> Vec<Vec<i32>> {
    let height = grid.len();
    let width = grid[0].len();
    let (r, c) = pos;
    let below = (r + 1..height)
        .zip(repeat(c))
        .map(|(x, y)| grid[x][y])
        .collect();
    let above = (0..r)
        .rev()
        .zip(repeat(c))
        .map(|(x, y)| grid[x][y])
        .collect();
    let right = repeat(r)
        .zip(c + 1..width)
        .map(|(x, y)| grid[x][y])
        .collect();
    let left = repeat(r)
        .zip((0..c).rev())
        .map(|(x, y)| grid[x][y])
        .collect();
    vec![above, below, right, left]
}

#[test]
fn test_scenic_score() {
    let input = include_str!("../example.txt");
    let grid = parse_input(input.as_bytes());
    assert_eq!(scenic_score((0, 0), &grid), 0);
    assert_eq!(scenic_score((1, 2), &grid), 4);
    assert_eq!(scenic_score((3, 2), &grid), 8);
}
