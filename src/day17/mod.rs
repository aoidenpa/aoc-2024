use std::collections::{HashMap, HashSet};
use itertools::*;

use crate::util::nums;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day17.txt"};
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
    let lines: Vec<&str> = input.lines().collect();
    let ra = nums::<i64>(lines[0])[0];
    let rb = nums::<i64>(lines[1])[0];
    let rc = nums::<i64>(lines[2])[0];
    let mut regs = [ra, rb, rc];
    let a = 0;
    let b = 1;
    let c = 2;
    let codes = nums::<i64>(lines[4]);
    let mut p = 0;
    loop {
	if p >= codes.len() {
	    break;
	}
	let code = codes[p];
	let op = codes[p+1];
	let mut inc = true;
	match code {
	    0 => {
		regs[a] = regs[a] / (1 << combo(op, regs));
	    },
	    1 => {
		regs[b] = regs[b] ^ op;
	    },
	    2 => {
		regs[b] = combo(op, regs) % 8;
	    },
	    3 => {
		if regs[a] != 0 {
		    p = op as usize;
		    inc = false;
		}
	    },
	    4 => {
		regs[b] = regs[b] ^ regs[c];
	    },
	    5 => {
		print!("{},", combo(op, regs) % 8);
	    },
	    6 => {
		regs[b] = regs[a] / (1 << combo(op, regs));
	    },
	    7 => {
		regs[c] = regs[a] / (1 << combo(op, regs));
	    },
	    _ => panic!()
	}
	if inc {
	    p += 2;
	}
    }
    "".to_string()
}

pub fn part2(input: String) -> String {
    let res;
    let lines: Vec<&str> = input.lines().collect();
    let codes = nums::<i64>(lines[4]);
    let mut set = HashSet::new();
    set.insert(0);
    for (ind, target) in codes.into_iter().rev().enumerate() {
	let mut newset = HashSet::new();
	for ra in set {
	    for i in 0..8 {
		let new_ra = ra * 8 + i;
		//using i as the last three bits, this is the number program outputs. if it is the one we want, add new possible value for register A to the set.
		let out = (new_ra >> (i ^ 3) ^ i) % 8;
		if out == target {
		    if !(ind == 0 && target == 0 && i == 0) {
			newset.insert(new_ra);
		    }
		}
	    }
	}
	set = newset;
    }
    let mut nums: Vec<i64> = set.into_iter().collect();
    nums.sort();
    res = nums[0];
    res.to_string()
}

fn combo(n: i64, regs: [i64; 3]) -> i64 {
    if n < 4 {
	n
    }
    else if n < 7 {
	regs[n as usize - 4]
    }
    else {
	panic!("combo 7 ");
    }
}



