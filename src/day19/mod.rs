use std::collections::{HashMap, HashSet};
use itertools::*;
use regex::Regex;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day19.txt"};
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
    let towels = input.split("\n\n").next().unwrap().split(", ").join("|");
    let pre = String::from(r"^(");
    let suf = String::from(r")+$");
    // for towels ab, cd, regex is "^(ab | cd)*$"
    let regst =  pre + &towels + &suf;
    let re = Regex::new(&regst).unwrap();
    let pats = input.split("\n\n").nth(1).unwrap();
    for line in pats.lines() {
	if re.is_match(line) {
	    res += 1;
	}
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let towels = input.split("\n\n").next().unwrap().split(", ").collect::<Vec<&str>>();
    let pats = input.split("\n\n").nth(1).unwrap();
    let mut map = HashMap::new();
    for line in pats.lines() {
	res += count(&towels, line, &mut map);
    }
    res.to_string()
}

fn count<'a>(towels: &Vec<&str>, s: &'a str, map: &mut HashMap<&'a str, i64>) -> i64 {
    if s == "" {
	return 1;
    }
    if let Some(r) = map.get(s) {
	return *r;
    }
    let mut res = 0;
    for &t in towels {
	if s.starts_with(t) {
	    res += count(towels, &s[t.len()..], map);
	}
    }
    map.insert(s, res);
    res
}
