#[derive(Debug, Clone)]
pub struct Monkey {
    pub inspections: u64,
    pub items: Vec<u64>,
    pub operation: Box<fn(u64) -> u64>,
    pub test_divisible_by: u64,
    pub if_true_throw_to: usize,
    pub if_false_throw_to: usize,
}

pub fn example() -> Vec<Monkey> {
    vec![
        Monkey {
            inspections: 0,
            items: vec![79, 98],
            operation: Box::new(|old| old * 19),
            test_divisible_by: 23,
            if_true_throw_to: 2,
            if_false_throw_to: 3,
        },
        Monkey {
            inspections: 0,
            items: vec![54, 65, 75, 74],
            operation: Box::new(|old| old + 6),
            test_divisible_by: 19,
            if_true_throw_to: 2,
            if_false_throw_to: 0,
        },
        Monkey {
            inspections: 0,
            items: vec![79, 60, 97],
            operation: Box::new(|old| old * old),
            test_divisible_by: 13,
            if_true_throw_to: 1,
            if_false_throw_to: 3,
        },
        Monkey {
            inspections: 0,
            items: vec![74],
            operation: Box::new(|old| old + 3),
            test_divisible_by: 17,
            if_true_throw_to: 0,
            if_false_throw_to: 1,
        },
    ]
}

pub fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            inspections: 0,
            items: vec![71, 86],
            operation: Box::new(|old| old * 13),
            test_divisible_by: 19,
            if_true_throw_to: 6,
            if_false_throw_to: 7,
        },
        Monkey {
            inspections: 0,
            items: vec![66, 50, 90, 53, 88, 85],
            operation: Box::new(|old| old + 3),
            test_divisible_by: 2,
            if_true_throw_to: 5,
            if_false_throw_to: 4,
        },
        Monkey {
            inspections: 0,
            items: vec![97, 54, 89, 62, 84, 80, 63],
            operation: Box::new(|old| old + 6),
            test_divisible_by: 13,
            if_true_throw_to: 4,
            if_false_throw_to: 1,
        },
        Monkey {
            inspections: 0,
            items: vec![82, 97, 56, 92],
            operation: Box::new(|old| old + 2),
            test_divisible_by: 5,
            if_true_throw_to: 6,
            if_false_throw_to: 0,
        },
        Monkey {
            inspections: 0,
            items: vec![50, 99, 67, 61, 86],
            operation: Box::new(|old| old * old),
            test_divisible_by: 7,
            if_true_throw_to: 5,
            if_false_throw_to: 3,
        },
        Monkey {
            inspections: 0,
            items: vec![61, 66, 72, 55, 64, 53, 72, 63],
            operation: Box::new(|old| old + 4),
            test_divisible_by: 11,
            if_true_throw_to: 3,
            if_false_throw_to: 0,
        },
        Monkey {
            inspections: 0,
            items: vec![59, 79, 63],
            operation: Box::new(|old| old * 7),
            test_divisible_by: 17,
            if_true_throw_to: 2,
            if_false_throw_to: 7,
        },
        Monkey {
            inspections: 0,
            items: vec![55],
            operation: Box::new(|old| old + 7),
            test_divisible_by: 3,
            if_true_throw_to: 2,
            if_false_throw_to: 1,
        },
    ]
}
