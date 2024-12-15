use std::collections::{HashMap, HashSet};
use itertools::*;
use crate::util::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day15.txt"};
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
    let map = input.split("\n\n").next().unwrap();
    let codes = input.split("\n\n").nth(1).unwrap();
    for line in map.lines() {
	grid.push(line.chars().collect::<Vec<char>>());
    }
    let codes = codes.replace("\n", "");
    let dirmap = HashMap::from([('v', (0, 1)), ('>', (1, 0)), ('<', (-1, 0)), ('^', (0, -1))]);
    let w = grid[0].len();
    let h = grid.len();
    let mut pos = (0, 0);
    'out: for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == '@' {
		pos = (i as i32, j as i32);
		grid[j][i] = '.';
		break 'out;
	    }
	}
    }
    for code in codes.chars() {
	let &dir = dirmap.get(&code).unwrap();
	let mut boxes = vec![];
	let mut ok = false;
	let mut n = pos;
	loop {
	    n = (n.0 + dir.0, n.1 + dir.1);
	    if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] == '#' {
		break;
	    }
	    if grid[n.1 as usize][n.0 as usize] == '.' {
		ok = true;
		break;
	    }
	    else {
		boxes.push(n);
	    }
	}
	if ok {
	    if !boxes.is_empty() {
		grid[boxes[0].1 as usize][boxes[0].0 as usize] = '.';
		let last = boxes.len() - 1;
		grid[(boxes[last].1 + dir.1) as usize][(boxes[last].0 + dir.0) as usize] = 'O';
	    }
	    pos = (pos.0 + dir.0, pos.1 + dir.1);
	}
    }
    for i in 0..h {
	for j in 0..w {
	    if grid[i][j] == 'O' {
		res += i*100 + j;
	    }
	}
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid = vec![];
    let map = input.split("\n\n").next().unwrap();
    let codes = input.split("\n\n").nth(1).unwrap();
    for line in map.lines() {
	let mut v = vec![];
	for c in line.chars() {
	    if c == '#' {
		v.push('#');
		v.push('#');
	    }
	    else if c == '.' {
		v.push('.');
		v.push('.');
	    }
	    else if c == 'O' {
		v.push('[');
		v.push(']');
	    }
	    else {
		v.push('@');
		v.push('.');
	    }
	}
	grid.push(v);
    }
    let codes = codes.replace("\n", "");
    let dirmap = HashMap::from([('v', (0, 1)), ('>', (1, 0)), ('<', (-1, 0)), ('^', (0, -1))]);
    let w = grid[0].len();
    let h = grid.len();
    let mut pos = (0, 0);
    'out: for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == '@' {
		pos = (i as i32, j as i32);
		grid[j][i] = '.';
		break 'out;
	    }
	}
    }
    let mut print = false;
    if print {
	for i in 0..h {
	    for j in 0..w {
		if (j as i32, i as i32) == pos {
		    print!("@");
		}
		else {
		    print!("{}", grid[i][j]);
		}
	    }
	    println!();
	}
    }
    // do a BFS on every box we touch. if any of them gets blocked, abort. if all ok, move all.
    for code in codes.chars() {
	let &dir = dirmap.get(&code).unwrap();
	let mut boxes = vec![];
	let mut ok = true;
	let mut pushes: Vec<(i32, i32)> = vec![pos];
	while !pushes.is_empty() {
	    let cur = pushes.remove(0);
	    if grid[cur.1 as usize][cur.0 as usize] == '[' || grid[cur.1 as usize][cur.0 as usize] == ']' {
		boxes.push(cur);
	    }
	    let n = (cur.0 + dir.0, cur.1 + dir.1);
	    if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] == '#' {
		ok = false;
		break;
	    }
	    if grid[n.1 as usize][n.0 as usize] == '[' {
		if dir.0 == 0 {
		    pushes.push((n.0, n.1));
		    pushes.push((n.0+1, n.1));
		}
		else {
		    pushes.push(n);
		}
	    }
	    else if grid[n.1 as usize][n.0 as usize] == ']' {
		if dir.0 == 0 {
		    pushes.push((n.0, n.1));
		    pushes.push((n.0-1, n.1));
		}
		else {
		    pushes.push(n);
		}
	    }
	    pushes = pushes.into_iter().unique().collect();
	}
	//if moving, move all encountered boxes and robot in dir
	if ok {
	    let boxes: Vec<(i32, i32)> = boxes.into_iter().unique().collect();
	    //since we did BFS, the farthest boxes are at the end. move them first.
	    for i in (0..boxes.len()).rev() {
		let b = boxes[i];
		grid[(b.1 + dir.1) as usize][(b.0 + dir.0) as usize] = grid[b.1 as usize][b.0 as usize];
		grid[b.1 as usize][b.0 as usize] = '.';
	    }
	    pos = (pos.0 + dir.0, pos.1 + dir.1);
	}
	if print {
	    for i in 0..h {
		for j in 0..w {
		    if (j as i32, i as i32) == pos {
			print!("@");
		    }
		    else {
			print!("{}", grid[i][j]);
		    }
		}
		println!();
	    }
	}
    }
    for i in 0..h {
	for j in 0..w {
	    if grid[i][j] == '[' {
		res += i*100 + j;
	    }
	}
    }
    res.to_string()
}
