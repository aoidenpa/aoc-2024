use std::collections::{HashMap, HashSet};
use itertools::*;
use std::{thread, time};

use crate::util::nums;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day14.txt"};
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
    let mut res = 1;
    let w = 101;
    let h = 103;
    let mut quads = vec![0; 4];
    for line in input.lines() {
	let nums = nums::<i64>(line);
	let (px, py) = (nums[0], nums[1]);
	let (vx, vy) = (nums[2], nums[3]);
	let mut nx = (px + vx * 100) % w;
	let mut ny = (py + vy * 100) % h;
	if nx < 0 {
	    nx += w;
	}
	if ny < 0 {
	    ny += h;
	}
	let midx = w / 2;
	let midy = h / 2;
	if nx < midx {
	    if ny < midy {
		quads[0] += 1;
	    }
	    else if ny > midy {
		quads[2] += 1;
	    }
	}
	else if nx > midx {
	    if ny < midy {
		quads[1] += 1;
	    }
	    else if ny > midy {
		quads[3] += 1;
	    }
	}
    }
    for q in quads {
	res *= q;
    }
    res.to_string()
}


// this prints iterations with 0.2 delay. one way to get the answer is realize there is a vertical and horizontal groupings. they start at an offset and repeat, one every 101 secs, other every 103 secs. finding the second they meet is the answer. there are other ways, like measuring how likely a grid has a tree in it, by measuring how close robots are.
pub fn part2(input: String) -> String {
    let mut res = 1;
    let w = 101;
    let h = 103;
    let mut robs = vec![];
    for line in input.lines() {
	let nums = nums::<i64>(line);
	let (px, py) = (nums[0], nums[1]);
	let (vx, vy) = (nums[2], nums[3]);
	robs.push(((px, py), (vx, vy)));
    }
    let mut secs = 0;
    loop {
	let mut grid = vec![vec![' ' ;w as usize];h as usize];
	for (p, v) in &mut robs {
	    let mut nx = (p.0 + v.0) % w;
	    let mut ny = (p.1 + v.1) % h;
	    if nx < 0 {
		nx += w;
	    }
	    if ny < 0 {
		ny += h;
	    }
	    p.0 = nx;
	    p.1 = ny;
	    grid[ny as usize][nx as usize] = '#';
	}
	secs += 1;
	println!("SECOND: {}", secs);
	for j in 0..h as usize {
	    for i in 0..w as usize {
		print!("{}", grid[j][i]);
	    }
	    println!();
	}
	println!();
	println!();
	thread::sleep(time::Duration::from_secs_f32(0.2));
	if secs > 101*103 {
	    break;
	}
    }
    res.to_string()
}

