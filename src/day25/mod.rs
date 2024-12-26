use std::collections::{HashMap, HashSet};
use itertools::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day25.txt"};
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
    let mut res = 0;
    let mut locks = vec![];
    let mut keys = vec![];
    for part in input.split("\n\n") {
        let mut g = vec![];
        for line in part.lines() {
            g.push(line.chars().collect::<Vec<char>>());
        }
        let mut pins = vec![];
        for j in 0..g[0].len() {
            let mut count = 0;
            for i in 1..g.len()-1 {
                if g[i][j] == '#' {
                    count += 1;
                }
            }
            pins.push(count);
        }
        if g[0][0] == '#' {
            locks.push(pins);
        }
        else {
            keys.push(pins);
        }
    }
    for lock in &locks {
        for key in &keys {
            let mut ok = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] > 5 {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += 1;
            }
        }
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    // click the button!
    // happy christmas.
    "end".to_string()
}
