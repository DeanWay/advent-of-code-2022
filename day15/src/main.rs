use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use regex::Regex;

fn main() {
    let input = stdin().lock();
    let sensors = parse_input(input);
    let coverage_by_row = all_coverage_by_row(&sensors);
    println!("{}", solution_1(&coverage_by_row, 2000000));
    println!("{:?}", solution_2(&coverage_by_row));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    closest_beacon: Position,
}

impl Sensor {
    fn manhatan_distance_to_beacon(&self) -> i32 {
        (self.position.x.abs_diff(self.closest_beacon.x)
            + self.position.y.abs_diff(self.closest_beacon.y)) as i32
    }

    #[allow(dead_code)]
    fn perimiter(&self) -> Vec<Position> {
        let mut res = Vec::new();
        let distance_to_beacon = self.manhatan_distance_to_beacon();
        let mut current_position = Position {
            x: self.position.x,
            y: self.position.y + distance_to_beacon as i32,
        };
        for _ in 0..distance_to_beacon {
            current_position.x -= 1;
            current_position.y -= 1;
            res.push(current_position.clone());
        }
        for _ in 0..distance_to_beacon {
            current_position.x += 1;
            current_position.y -= 1;
            res.push(current_position.clone());
        }
        for _ in 0..distance_to_beacon {
            current_position.x += 1;
            current_position.y += 1;
            res.push(current_position.clone());
        }
        for _ in 0..distance_to_beacon {
            current_position.x -= 1;
            current_position.y += 1;
            res.push(current_position.clone());
        }
        res
    }

    fn perimeter_by_row(&self) -> HashMap<i32, (i32, i32)> {
        let mut row_to_coverage_range = HashMap::new();
        for i in 0..=self.manhatan_distance_to_beacon() {
            let row = self.position.y + (self.manhatan_distance_to_beacon() - i);
            let coverage_range = (self.position.x - i, self.position.x + i);
            row_to_coverage_range.insert(row, coverage_range);
        }
        for i in 0..self.manhatan_distance_to_beacon() {
            let row = self.position.y - (self.manhatan_distance_to_beacon() - i);
            let coverage_range = (self.position.x - i, self.position.x + i);
            row_to_coverage_range.insert(row, coverage_range);
        }
        row_to_coverage_range
    }
}

fn parse_input(input: impl BufRead) -> Vec<Sensor> {
    let pattern =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let captures = pattern.captures(&line).unwrap();
            let sensor_x = captures[1].parse().unwrap();
            let sensor_y = captures[2].parse().unwrap();
            let beacon_x = captures[3].parse().unwrap();
            let beacon_y = captures[4].parse().unwrap();
            Sensor {
                position: Position {
                    x: sensor_x,
                    y: sensor_y,
                },
                closest_beacon: Position {
                    x: beacon_x,
                    y: beacon_y,
                },
            }
        })
        .collect()
}

fn all_coverage_by_row(sensors: &[Sensor]) -> HashMap<i32, Vec<(i32, i32)>> {
    let mut out = HashMap::new();
    for sensor in sensors {
        let coverage_by_row = sensor.perimeter_by_row();
        for (row, coverage) in coverage_by_row {
            match out.get_mut(&row) {
                None => {
                    out.insert(row, vec![coverage]);
                }
                Some(existing_row_coverage) => {
                    existing_row_coverage.push(coverage);
                }
            };
        }
    }
    for row_coverage in out.values_mut() {
        row_coverage.sort();
        *row_coverage = row_coverage.iter().fold(Vec::new(), |mut acc, current| {
            match acc.last_mut() {
                None => acc.push(current.clone()),
                Some(last) => {
                    if last.1 + 1 >= current.0 {
                        if last.1 <= current.1 {
                            last.1 = current.1
                        }
                    } else if last.1 < current.0 {
                        acc.push(current.clone())
                    }
                }
            };
            acc
        });
    }
    out
}

fn solution_1(coverage: &HashMap<i32, Vec<(i32, i32)>>, row: i32) -> i32 {
    let sum_covered_area = coverage
        .get(&row)
        .unwrap()
        .iter()
        .fold(0, |acc, range| acc + (range.1 - range.0));
    sum_covered_area
}

fn solution_2(coverage: &HashMap<i32, Vec<(i32, i32)>>) -> Vec<i128> {
    let max_row = 4000000;
    let mut out = Vec::new();
    for row in (0..=max_row).rev() {
        match coverage.get(&row) {
            None => {
                continue;
            }
            Some(row_coverage) => {
                if row_coverage.len() == 2 {
                    let x = row_coverage[0].1 + 1;
                    out.push(x as i128 * 4000000i128 + row as i128)
                }
            }
        }
    }
    out
}

#[test]
fn test_perimeter() {
    let sensor = Sensor {
        position: Position { x: 0, y: 0 },
        closest_beacon: Position { x: 1, y: 1 },
    };
    assert_eq!(
        sensor.perimiter(),
        vec![
            Position { x: -1, y: 1 },
            Position { x: -2, y: 0 },
            Position { x: -1, y: -1 },
            Position { x: 0, y: -2 },
            Position { x: 1, y: -1 },
            Position { x: 2, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 0, y: 2 },
        ]
    );
}

#[test]
fn test_perimeter_by_row() {
    let sensor = Sensor {
        position: Position { x: 0, y: 0 },
        closest_beacon: Position { x: 1, y: 1 },
    };
    assert_eq!(
        sensor.perimeter_by_row(),
        HashMap::from_iter([
            (2, (0, 0)),
            (1, (-1, 1)),
            (0, (-2, 2)),
            (-1, (-1, 1)),
            (-2, (0, 0)),
        ])
    );
}
