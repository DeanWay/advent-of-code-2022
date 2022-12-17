use std::{
    fmt::Display,
    io::{stdin, BufRead},
    iter::successors,
    vec,
};

fn main() {
    let input = stdin().lock();
    let rock_paths = parse_input(input);
    println!("solution 1: {}", solution_1(&rock_paths));
    println!("solution 2: {}", solution_2(&rock_paths));
}

fn solution_1(rock_paths: &[Path]) -> usize {
    let cave = Cave::from_rock_paths(rock_paths);
    cave.drop_sand_until_all_settles()
        .last()
        .unwrap()
        .count_grains_of_sand()
}

fn solution_2(rock_paths: &[Path]) -> usize {
    let ((min_col, max_col), (_, max_depth)) = bounds(rock_paths);
    let mut rock_paths = rock_paths.to_owned();
    rock_paths.push(vec![
        Point {
            depth: max_depth + 2,
            col: min_col - max_depth,
        },
        Point {
            depth: max_depth + 2,
            col: max_col + max_depth,
        },
    ]);

    let cave = Cave::from_rock_paths(&rock_paths);
    cave.drop_sand_until_all_settles()
        .last()
        .unwrap()
        .count_grains_of_sand()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    depth: usize,
    col: usize,
}

impl Point {
    fn one_down(&self) -> Self {
        Self {
            depth: self.depth + 1,
            col: self.col,
        }
    }

    fn one_down_left(&self) -> Option<Self> {
        let col = self.col.checked_sub(1)?;
        Some(Self {
            depth: self.depth + 1,
            col,
        })
    }

    fn one_down_right(&self) -> Self {
        Self {
            depth: self.depth + 1,
            col: self.col + 1,
        }
    }
}

type Path = Vec<Point>;

fn parse_input(input: impl BufRead) -> Vec<Path> {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let (col, depth) = coord.split_once(',').unwrap();
                    let col = col.parse().unwrap();
                    let depth = depth.parse().unwrap();
                    Point { depth, col }
                })
                .collect()
        })
        .collect()
}

fn bounds(rock_paths: &[Path]) -> ((usize, usize), (usize, usize)) {
    let min_col = rock_paths
        .iter()
        .flatten()
        .map(|point| point.col)
        .min()
        .unwrap();
    let max_col = rock_paths
        .iter()
        .flatten()
        .map(|point| point.col)
        .max()
        .unwrap();
    let max_depth = rock_paths
        .iter()
        .flatten()
        .map(|point| point.depth)
        .max()
        .unwrap();
    ((min_col, max_col), (0, max_depth))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    SandSpout,
    Empty,
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
struct Cave {
    height: usize,
    width: usize,
    min_col: usize,
    grid: Vec<Vec<Cell>>,
}

impl Cave {
    fn from_rock_paths(rock_paths: &[Path]) -> Self {
        let ((min_col, max_col), (min_depth, max_depth)) = bounds(rock_paths);
        let mut grid = Vec::new();
        for _ in min_depth..=max_depth {
            let mut row = Vec::new();
            for _ in min_col..=max_col {
                row.push(Cell::Empty);
            }
            grid.push(row);
        }
        for path in rock_paths {
            for window in path.windows(2) {
                let a = &window[0];
                let b = &window[1];
                if a.col == b.col {
                    let (min, max) = if a.depth < b.depth {
                        (a.depth, b.depth)
                    } else {
                        (b.depth, a.depth)
                    };
                    for depth in min..=max {
                        grid[depth][a.col - min_col] = Cell::Rock;
                    }
                } else if a.depth == b.depth {
                    let (min, max) = if a.col < b.col {
                        (a.col, b.col)
                    } else {
                        (b.col, a.col)
                    };
                    for col in min..=max {
                        grid[a.depth][col - min_col] = Cell::Rock;
                    }
                } else {
                    panic!()
                }
            }
        }
        grid[0][500 - min_col] = Cell::SandSpout;
        let height = grid.len();
        let width = grid[0].len();
        Self {
            height,
            width,
            min_col,
            grid,
        }
    }

    fn sand_spout_pos(&self) -> Point {
        Point {
            depth: 0,
            col: 500 - self.min_col,
        }
    }

    fn in_bound(&self, point: &Point) -> bool {
        point.depth < self.height && point.col < self.width
    }

    fn drop_grain_of_sand(&self) -> Option<Cave> {
        let mut current_pos = self.sand_spout_pos();
        if self.grid[current_pos.depth][current_pos.col] == Cell::Sand {
            println!("reached the top!");
            return None;
        }
        let mut next_cave = self.clone();
        loop {
            let one_down = current_pos.one_down();
            if !self.in_bound(&one_down) {
                return None;
            }
            if self.grid[one_down.depth][one_down.col] == Cell::Empty {
                current_pos = one_down;
                continue;
            }
            let down_left = current_pos.one_down_left();
            if let Some(down_left) = down_left {
                if !self.in_bound(&down_left) {
                    return None;
                }
                if self.grid[down_left.depth][down_left.col] == Cell::Empty {
                    current_pos = down_left;
                    continue;
                }
            } else {
                return None;
            }
            let down_right = current_pos.one_down_right();
            if !self.in_bound(&down_right) {
                return None;
            }
            if self.grid[down_right.depth][down_right.col] == Cell::Empty {
                current_pos = down_right;
                continue;
            }
            next_cave.grid[current_pos.depth][current_pos.col] = Cell::Sand;
            break;
        }
        Some(next_cave)
    }

    fn drop_sand_until_all_settles(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(successors(Some(self.clone()), |current_cave| {
            current_cave.drop_grain_of_sand()
        }))
    }
    fn count_grains_of_sand(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|&&cell| cell == Cell::Sand)
            .fold(0, |acc, _| acc + 1)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let c = match &self.grid[r][c] {
                    Cell::SandSpout => '+',
                    Cell::Empty => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                };
                write!(f, "{}", c)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}
