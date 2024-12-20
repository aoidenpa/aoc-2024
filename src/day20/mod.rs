use std::collections::{HashMap, HashSet, VecDeque};
use itertools::*;

use crate::util::in_bounds;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day20.txt"};
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
    solve(input, 1)
}

pub fn part2(input: String) -> String {
    solve(input, 2)
}
fn solve(input: String, part: i32) -> String {
    let mut res = 0;
    let mut grid = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in input.lines().enumerate() {
	let v = line.chars().collect::<Vec<char>>();
	for (j, &c) in v.iter().enumerate() {
	    if c == 'S' {
		start = (j, i);
	    }
	    if c == 'E' {
		end = (j, i);
	    }
	}
	grid.push(v);
    }
    grid[start.1][start.0] = '.';
    grid[end.1][end.0] = '.';
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let w = grid[0].len();
    let h = grid.len();
    //find distances to end and to start for each node
    //to end
    let mut dist_end = vec![vec![i32::MAX; w]; h];
    let mut dist_start = vec![vec![i32::MAX; w]; h];
    let mut q = VecDeque::new();
    dist_end[end.1][end.0] = 0;
    q.push_back(end);
    while !q.is_empty() {
	let cur = q.pop_front().unwrap();
	for d in dirs {
	    let n = (cur.0 as i32 + d.0, cur.1 as i32 + d.1);
	    if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] == '#' {
		continue;
	    }
	    if dist_end[cur.1][cur.0] + 1 < dist_end[n.1 as usize][n.0 as usize] {
		dist_end[n.1 as usize][n.0 as usize] = dist_end[cur.1][cur.0] + 1;
		q.push_back((n.0 as usize, n.1 as usize));
	    }
	}
    }
    //to start
    let mut q = VecDeque::new();
    dist_start[start.1][start.0] = 0;
    q.push_back(start);
    while !q.is_empty() {
	let cur = q.pop_front().unwrap();
	for d in dirs {
	    let n = (cur.0 as i32 + d.0, cur.1 as i32 + d.1);
	    if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] == '#' {
		continue;
	    }
	    if dist_start[cur.1][cur.0] + 1 < dist_start[n.1 as usize][n.0 as usize] {
		dist_start[n.1 as usize][n.0 as usize] = dist_start[cur.1][cur.0] + 1;
		q.push_back((n.0 as usize, n.1 as usize));
	    }
	}
    }
    let no_cheat = dist_end[start.1][start.0];
    let mut cheats = HashSet::new();
    let cheat_dist = if part == 1 {2} else {20};
    for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == '.' {
		let lmin = (i as i32 - cheat_dist).max(0) as usize;
		let rmin = (j as i32 - cheat_dist).max(0) as usize;
		let lmax = (i as i32 + cheat_dist).min(w as i32-1) as usize;
		let rmax = (j as i32 + cheat_dist).min(h as i32-1) as usize;
		for ii in lmin..=lmax {
		    for jj in rmin..=rmax {
			if grid[jj][ii] == '.' {
			    let dd = (i as i32-ii as i32).abs() + (j as i32-jj as i32).abs();
			    if dd > cheat_dist {
				continue;
			    }
			    let m = no_cheat - (dist_start[j][i] + dist_end[jj][ii] + dd);
			    if m >= 100 {
				if !cheats.contains(&((i, j), (ii, jj)))
				{
				    cheats.insert(((i, j), (ii, jj)));
				    res += 1;
				}
			    }
			}
		    }
		}
	    }
	}
    }
    res.to_string()
}
