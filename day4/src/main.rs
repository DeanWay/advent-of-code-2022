use std::io::stdin;

fn main() {
    let input = parse_input();
    println!("solution 1: {:?}", solution_1(&input));
    println!("solution 2: {:?}", solution_2(&input));
}

type Pair = (i32, i32);
type Input = Vec<(Pair, Pair)>;

fn parse_input() -> Input {
    stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(",").map(|range| {
                let mut split = range.split("-");
                let fst = split.next().unwrap().parse().unwrap();
                let snd = split.next().unwrap().parse().unwrap();
                (fst, snd)
            });
            let fst = split.next().unwrap();
            let snd = split.next().unwrap();
            (fst, snd)
        })
        .collect()
}

fn solution_1(input: &Input) -> i32 {
    input
        .iter()
        .filter(|(a, b)| one_contains_other(a, b))
        .fold(0, |acc, _| acc + 1)
}

fn one_contains_other(a: &Pair, b: &Pair) -> bool {
    let contains = |x: &Pair, y: &Pair| x.0 <= y.0 && x.1 >= y.1;
    contains(a, b) || contains(b, a)
}

fn solution_2(input: &Input) -> i32 {
    input
        .iter()
        .filter(|(a, b)| have_any_overlap(a, b))
        .fold(0, |acc, _| acc + 1)
}

fn have_any_overlap(a: &Pair, b: &Pair) -> bool {
    let overlap = |x: &Pair, y: &Pair| x.0 <= y.0 && x.1 >= y.0;
    one_contains_other(a, b) || overlap(a, b) || overlap(b, a)
}

#[test]
fn test_have_any_overlap() {
    assert!(have_any_overlap(&(1, 3), &(2, 4)));
    assert!(have_any_overlap(&(1, 1), &(1, 1)));
    assert!(have_any_overlap(&(1, 4), &(2, 3)));
    assert!(have_any_overlap(&(2, 3), &(1, 4)));
    assert!(have_any_overlap(&(1, 4), &(2, 3)));
    assert!(have_any_overlap(&(1, 5), &(5, 20)));

    assert_eq!(have_any_overlap(&(1, 1), &(2, 2)), false);
    assert_eq!(have_any_overlap(&(1, 5), &(9, 20)), false);
    assert_eq!(have_any_overlap(&(2, 2), &(1, 1)), false);
}
