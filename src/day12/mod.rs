use std::collections::{HashMap, HashSet};
use itertools::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day12.txt"};
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
    let mut grid = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect::<Vec<char>>());
    }
    let w = grid[0].len();
    let h = grid.len();
    let mut seen = HashSet::new();
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for i in 0..w {
	for j in 0..h {
	    if seen.contains(&(i, j)) {
		continue;
	    }
	    let c  = grid[j][i];
	    let mut q = vec![];
	    q.push((i, j));
	    let mut peri = 0;
	    let mut area = 0;
	    while !q.is_empty() {
		area += 1;
		let cur = q.pop().unwrap();
		seen.insert(cur);
		for dir in dirs {
		    let n = (cur.0 as i32 + dir.0, cur.1 as i32 + dir.1);
		    if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] != c {
			peri += 1;
		    }
		    if in_bounds(n, (w, h)) && grid[n.1 as usize][n.0 as usize] == c {
			let n = (n.0 as usize, n.1 as usize);
			if !seen.contains(&n) {
			    seen.insert(n);
			    q.push(n);
			}
		    }
		}
	    }
	    res += peri * area;
	}
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect::<Vec<char>>());
    }
    let w = grid[0].len();
    let h = grid.len();
    let mut seen = HashSet::new();
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for i in 0..w {
	for j in 0..h {
	    if seen.contains(&(i, j)) {
		continue;
	    }
	    seen.insert((i, j));
	    let c  = grid[j][i];
	    let mut q = vec![];
	    q.push((i, j));
	    let mut peri = 0;
	    let mut area = 0;
	    while !q.is_empty() {
		let cur = q.pop().unwrap();
		area += 1;
		peri += get_corners(&grid, cur);
		for dir in dirs {
		    let n = (cur.0 as i32 + dir.0, cur.1 as i32 + dir.1);
		    if in_bounds(n, (w, h)) && grid[n.1 as usize][n.0 as usize] == c {
			let n = (n.0 as usize, n.1 as usize);
			if !seen.contains(&n) {
			    seen.insert(n);
			    q.push(n);
			}
		    }
		}
	    }
	    res += peri * area;
	}
    }
    res.to_string()
}
// calculate how many corners are on cur. number of corners is equal to the number of sides.
fn get_corners(grid: &Vec<Vec<char>>, cur: (usize, usize)) -> i32 {
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0), (0, -1)];
    let c = grid[cur.1][cur.0];
    let w = grid[0].len();
    let h = grid.len();
    let mut corners = 0;
    for k in 0..4 {
	let c1 = (cur.0 as i32 + dirs[k].0, cur.1 as i32 + dirs[k].1);
	let c2 = (cur.0 as i32 + dirs[k+1].0, cur.1 as i32 + dirs[k+1].1);
	let c3 = (cur.0 as i32 + dirs[k].0 + dirs[k+1].0, cur.1 as i32 + dirs[k].1 + dirs[k+1].1);
	// ex: if top and right of cur is empty, this is a top right corner. do that for all 4 corners
	if (!in_bounds(c1, (w, h)) || grid[c1.1 as usize][c1.0 as usize] != c) &&
	    (!in_bounds(c2, (w, h)) || grid[c2.1 as usize][c2.0 as usize] != c) {
		corners += 1;
	    }
	// ex: if top and right of cur is filled and top-right of cur is empty, this is a concave corner.
	if (in_bounds(c1, (w, h)) && grid[c1.1 as usize][c1.0 as usize] == c) &&
	    (in_bounds(c2, (w, h)) && grid[c2.1 as usize][c2.0 as usize] == c) &&
	    grid[c3.1 as usize][c3.0 as usize] != c {
		corners += 1;
	    }
    }
    corners
}
fn in_bounds(cur: (i32, i32), size: (usize, usize)) -> bool {
    cur.0 >= 0 && cur.0 < size.0 as i32 && cur.1 >= 0 && cur.1 < size.1 as i32
}
