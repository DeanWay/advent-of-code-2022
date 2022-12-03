use std::{collections::HashSet, io::stdin};

type Input = Vec<Vec<char>>;

fn main() {
    let input = parse_input();
    println!("{:?}", solution_1(&input));
    println!("{:?}", solution_2(&input));
}

fn parse_input() -> Input {
    stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn solution_1(input: &Input) -> i32 {
    input
        .iter()
        .map(|bag| {
            let (fst, snd) = bag.split_at(bag.len() / 2);
            let fst_set = HashSet::<&char>::from_iter(fst);
            let snd_set = HashSet::<&char>::from_iter(snd);
            *fst_set.intersection(&snd_set).next().unwrap()
        })
        .map(priority)
        .sum()
}

fn solution_2(input: &Input) -> i32 {
    input
        .chunks_exact(3)
        .map(|group| {
            let a_set = HashSet::<&char>::from_iter(group[0].iter());
            let b_set = HashSet::<&char>::from_iter(group[1].iter());
            let c_set = HashSet::<&char>::from_iter(group[2].iter());
            *a_set
                .iter()
                .filter(|x| b_set.contains(*x))
                .filter(|x| c_set.contains(*x))
                .next()
                .unwrap()
        })
        .map(priority)
        .sum()
}

fn priority(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 'a' as i32 + 1,
        'A'..='Z' => *c as i32 - 'A' as i32 + 27,
        _ => panic!(),
    }
}

#[test]
fn test_priority() {
    assert_eq!(priority(&'a'), 1);
    assert_eq!(priority(&'b'), 2);
    assert_eq!(priority(&'z'), 26);
    assert_eq!(priority(&'A'), 27);
    assert_eq!(priority(&'B'), 28);
    assert_eq!(priority(&'Z'), 52);
}
