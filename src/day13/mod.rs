use crate::util::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day13.txt"};
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
    solve(input, 1).to_string()
}

pub fn part2(input: String) -> String {
    solve(input, 2).to_string()
}

fn solve(input: String, part: i32) -> i64 {
    let mut res = 0;
    let v = input.lines().collect::<Vec<&str>>();
    for chunk in v.chunks(4) {
	let numsa: Vec<i64> = nums(chunk[0]);
	let numsb: Vec<i64> = nums(chunk[1]);
	let numsp: Vec<i64> = nums(chunk[2]);
	let a = (numsa[0], numsa[1]);
	let b = (numsb[0], numsb[1]);
	let mut p = (numsp[0], numsp[1]);
	if part == 2 {
	    p.0 += 10000000000000;
	    p.1 += 10000000000000;
	}
	let top = p.1*b.0 - p.0*b.1;
	let bot = a.1*b.0 - b.1*a.0;
	let atimes = top / bot;
	let va = (a.0 * atimes, a.1 * atimes);
	let btimes = (p.0 - va.0) / b.0;
	if a.0 * atimes + b.0 * btimes == p.0 && a.1 * atimes + b.1 * btimes == p.1 {
	    res += atimes * 3 + btimes;
	}
    }
    res
}
