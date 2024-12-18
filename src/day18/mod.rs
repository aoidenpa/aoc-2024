use std::collections::{HashMap, HashSet, VecDeque};
use itertools::*;

use crate::util::{in_bounds, nums};

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day18.txt"};
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
    let w = 71;
    let h = 71;
    let mut grid = vec![vec![0; w]; h];
    let mut dist = vec![vec![-1; w]; h];
    for line in input.lines().take(1024) {
	let nums = nums::<i32>(line);
	let (x, y) = (nums[0] as usize, nums[1] as usize);
	grid[y][x] = 1;
    }
    let start = (0, 0);
    let end = (w - 1, h - 1);
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut q = VecDeque::new();
    q.push_back(start);
    dist[start.1][start.0] = 0;
    while !q.is_empty() {
	let (x, y) = q.pop_front().unwrap();
	if (x, y) == end {
	    res = dist[y][x];
	    break;
	}
	for d in dirs {
	    let (nx, ny) = (x as i32+d.0, y as i32+d.1);
	    if in_bounds((nx, ny), (w, h)) && dist[ny as usize][nx as usize] == -1 && grid[ny as usize][nx as usize] == 0 {
		dist[ny as usize][nx as usize] = dist[y][x] + 1;
		q.push_back((nx as usize, ny as usize));
	    }
	}
    }
    res.to_string()
}
// unimaginative brute force, good enough
pub fn part2(input: String) -> String {
    let mut res = String::new();
    let w = 71;
    let h = 71;
    let mut grid = vec![vec![0; w]; h];
    let mut dist = vec![vec![-1; w]; h];
    let start = (0, 0);
    let end = (w - 1, h - 1);
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut q = VecDeque::new();
    q.push_back(start);
    dist[start.1][start.0] = 0;
    for line in input.lines() {
	let nums = nums::<i32>(line);
	let (x, y) = (nums[0] as usize, nums[1] as usize);
	grid[y][x] = 1;
	let mut q = VecDeque::new();
	let mut seen = vec![vec![0; w]; h];
	q.push_back(start);
	let mut ok = false;
	while !q.is_empty() {
	    let (cx, cy) = q.pop_front().unwrap();
	    if (cx, cy) == end {
		ok = true;
		break;
	    }
	    if seen[cy][cx] == 1 {
		continue;
	    }
	    seen[cy][cx] = 1;
	    for d in dirs {
		let (nx, ny) = (cx as i32+d.0, cy as i32+d.1);
		if in_bounds((nx, ny), (w, h)) && grid[ny as usize][nx as usize] == 0 {
		    q.push_back((nx as usize, ny as usize));
		}
	    }
	}
	if !ok {
	    res = format!("{},{}", x, y);
	    break;
	}
    }
    res
    //res.to_string()
}
