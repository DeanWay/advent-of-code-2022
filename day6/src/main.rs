use std::{collections::HashSet, hash::Hash, io::stdin};

fn main() {
    let input = parse_input();
    println!("{:?}", solution_1(&input));
    println!("{:?}", solution_2(&input));
}

fn parse_input() -> String {
    stdin().lines().map(|line| line.unwrap()).nth(0).unwrap()
}

fn solution_1(input: &String) -> Option<usize> {
    first_occurance_of_unique_seq_of_len(input, 4)
}

fn solution_2(input: &String) -> Option<usize> {
    first_occurance_of_unique_seq_of_len(&input, 14)
}

fn first_occurance_of_unique_seq_of_len(message: &String, len: usize) -> Option<usize> {
    let window_size = len;
    let chars: Vec<char> = message.chars().collect();
    for (i, window) in chars.windows(window_size).enumerate() {
        if has_unique_elems(window) {
            return Some(i + window_size);
        }
    }
    None
}

fn has_unique_elems<T: Hash + Eq>(v: &[T]) -> bool {
    HashSet::<&T>::from_iter(v.iter()).len() == v.len()
}
