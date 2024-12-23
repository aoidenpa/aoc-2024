use std::collections::{HashMap, HashSet};

const MOD: i64 = 16777216;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day22.txt"};
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
    for line in input.lines() {
	let mut n = line.parse::<i64>().unwrap();
	let m = n;
	for i in 0..2000 {
	    n = next(n);
	}
	res += n;
    }
    
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut map = HashMap::new();
    for line in input.lines() {
	let mut n = line.parse::<i64>().unwrap();
	let mut p = vec![n % 10];
	for i in 0..2000 {
	    n = next(n);
	    p.push(n % 10);
	}
	let mut seen = HashSet::new();
	for i in 4..p.len() {
	    let d = (p[i-3]-p[i-4], p[i-2]-p[i-3], p[i-1]-p[i-2], p[i]-p[i-1]);
	    let val = p[i];
	    if !seen.contains(&d) {
		let r = map.entry(d).or_insert(0);
		*r += val;
		res = res.max(*r);
	    }
	    seen.insert(d);
	}
    }
    res.to_string()
}


fn next(mut n: i64) -> i64 {
    n = ((n * 64) ^ n) % MOD;
    n = ((n / 32) ^ n) % MOD;
    n = ((n * 2048) ^ n) % MOD;
    n
}
