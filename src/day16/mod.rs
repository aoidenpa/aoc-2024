use std::collections::{HashMap, HashSet};
use itertools::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day16.txt"};
    let input = std::fs::read_to_string(path).unwrap();
    match part {
        1 => {
            part1(input)
        },
        2 => {
            part2(input)
        },
        _ => panic!()
    }
}

pub fn part1(input: String) -> String {
    "".to_string()
}

pub fn part2(input: String) -> String {
    "".to_string()
}
