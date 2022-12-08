use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::{
    io::{stdin, BufRead},
    rc::Rc,
    vec,
};

fn main() {
    let line_tokens = parse_input(stdin().lock());
    println!("{}", solution_1(&line_tokens));
    println!("{:?}", solution_2(&line_tokens));
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    parent: Option<Rc<RefCell<Directory>>>,
    subdirectories: Vec<Rc<RefCell<Directory>>>,
}

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: usize,
}

#[derive(Debug)]
enum LineToken {
    CD(String),
    CDOut,
    LS,
    Dir { name: String },
    File { size: usize, name: String },
}

fn parse_input(input: impl BufRead) -> Vec<LineToken> {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(
            |line| match line.split(" ").collect::<Vec<&str>>().as_slice() {
                ["$", "ls"] => LineToken::LS,
                ["$", "cd", ".."] => LineToken::CDOut,
                ["$", "cd", dirname] => LineToken::CD(dirname.to_string()),
                ["dir", dirname] => LineToken::Dir {
                    name: dirname.to_string(),
                },
                [size, filename] => LineToken::File {
                    size: size.parse().unwrap(),
                    name: filename.to_string(),
                },
                _ => panic!("unparsable line: {}", line),
            },
        )
        .collect()
}

fn solution_1(input: &Vec<LineToken>) -> usize {
    let top_dir = parse_filesystem(input);
    let iter = DirectoryIterator::new(top_dir);
    iter.map(|item| item.borrow().size())
        .filter(|size| *size < 100000)
        .sum()
}

fn solution_2(input: &Vec<LineToken>) -> Option<usize> {
    let top_dir = parse_filesystem(input);
    let iter = DirectoryIterator::new(top_dir);
    let total_space: usize = 70000000;
    let needed_space: usize = 30000000;
    let mut sizes = iter
        .map(|item| item.borrow().size())
        .collect::<Vec<usize>>();
    sizes.sort();
    let space_used: usize = *sizes.iter().last().unwrap();
    sizes
        .iter()
        .find(|size| {
            total_space
                .checked_sub(space_used.checked_sub(**size).unwrap_or(0))
                .map(|space_left| space_left >= needed_space)
                .unwrap_or(false)
        })
        .cloned()
}

impl Directory {
    fn size(&self) -> usize {
        let sum_file_sizes: usize = self.files.iter().map(|file| file.size).sum();
        let sum_subdirectory_sizes: usize = self
            .subdirectories
            .iter()
            .map(|dir| dir.borrow().size())
            .sum();
        sum_file_sizes + sum_subdirectory_sizes
    }
}

struct DirectoryIterator {
    queue: VecDeque<Rc<RefCell<Directory>>>,
}

impl DirectoryIterator {
    fn new(root: Rc<RefCell<Directory>>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_front(root);
        DirectoryIterator { queue }
    }
}

impl Iterator for DirectoryIterator {
    type Item = Rc<RefCell<Directory>>;

    fn next(&mut self) -> Option<Self::Item> {
        let front = self.queue.pop_front();
        if let Some(front) = front {
            for dir in front.borrow().subdirectories.iter() {
                self.queue.push_front(dir.clone());
            }
            Some(front)
        } else {
            None
        }
    }
}

fn parse_filesystem(input: &Vec<LineToken>) -> Rc<RefCell<Directory>> {
    let mut lines = input.iter();
    let _first_cd = lines.next();
    let top_level_dir = Rc::new(RefCell::new(Directory {
        name: "/".to_owned(),
        parent: None,
        files: vec![],
        subdirectories: vec![],
    }));
    let mut current = Some(top_level_dir.clone());
    for line in lines {
        match line {
            LineToken::CDOut => {
                match current.take() {
                    None => panic!(),
                    Some(curr) => {
                        current = curr.borrow().parent.clone();
                    }
                };
            }
            LineToken::CD(dirname) => {
                match current.take() {
                    None => panic!(),
                    Some(curr) => {
                        let current_borrow = curr.borrow();
                        let new_curr = current_borrow
                            .subdirectories
                            .iter()
                            .find(|dir| dir.borrow().name == *dirname)
                            .unwrap()
                            .clone();
                        current = Some(new_curr)
                    }
                };
            }
            LineToken::LS => {}
            LineToken::File { name, size } => {
                match &mut current {
                    None => panic!(),
                    Some(curr) => {
                        curr.borrow_mut().files.push(File {
                            name: name.clone(),
                            size: *size,
                        });
                    }
                };
            }
            LineToken::Dir { name } => {
                match &mut current {
                    None => panic!(),
                    Some(curr) => {
                        curr.borrow_mut()
                            .subdirectories
                            .push(Rc::new(RefCell::new(Directory {
                                name: name.clone(),
                                parent: Some(curr.clone()),
                                files: vec![],
                                subdirectories: vec![],
                            })));
                    }
                };
            }
        };
    }
    top_level_dir.to_owned()
}

#[test]
fn test_parse_filesystem() {
    let lines = vec![
        LineToken::CD("/".to_owned()),
        LineToken::LS,
        LineToken::Dir {
            name: "a".to_owned(),
        },
        LineToken::File {
            size: 14848514,
            name: "b.txt".to_owned(),
        },
        LineToken::Dir {
            name: "d".to_owned(),
        },
        LineToken::CD("a".to_owned()),
        LineToken::LS,
        LineToken::Dir {
            name: "e".to_owned(),
        },
        LineToken::File {
            size: 29116,
            name: "f".to_owned(),
        },
        LineToken::File {
            size: 2557,
            name: "g".to_owned(),
        },
        LineToken::File {
            size: 62596,
            name: "h.lst".to_owned(),
        },
    ];
    let result = parse_filesystem(&lines);
    {
        let top_dir = result.borrow();
        assert_eq!(top_dir.name, "/".to_owned());
        let a = &top_dir.subdirectories[0];
        let b = &top_dir.subdirectories[1];
    }
}

#[test]
fn test_parse_example_input() {
    let input = include_str!("../example.txt");
    let lines = parse_input(input.as_bytes() as &[u8]);
    let result = parse_filesystem(&lines);
    assert_eq!(1, 1)
}
