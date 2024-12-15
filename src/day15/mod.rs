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
    // println!("map: {:?}", grid);
    // println!("codes: {}", codes);
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
	    println!("{:?}", code);
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
	// println!("After {}: pos: {:?} \n", code, pos);
	// for i in 0..h {
	//     for j in 0..w {
	// 	if (j as i32, i as i32) == pos {
	// 	    print!("@");
	// 	}
	// 	else {
	// 	    print!("{}", grid[i][j]);
	// 	}
	//     }
	//     println!();
	// }
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
	//grid.push(line.chars().collect::<Vec<char>>());
	grid.push(v);
    }
    let codes = codes.replace("\n", "");
    // println!("map: {:?}", grid);
    // println!("codes: {}", codes);
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
    println!("INIT: ");
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
    for (e, code) in codes.chars().enumerate() {
	let &dir = dirmap.get(&code).unwrap();
	let mut boxes = vec![];
	let mut ok = false;
	let mut n = pos;
	let mut two: Option<(Vec<(i32, i32)>, i32)> = None;
	let mut print = false;
	loop {
	    if let Some((pushes, y)) = two {
		let mut check = true;
		let mut newtwo = vec![];
		for push in pushes {
		    let n1 = (push.0, y);
		    let n2 = (push.1, y);
		    if !in_bounds(n1, (w, h)) || !in_bounds(n2, (w, h)) {
			check = false;
			break;
		    }
		    let n1u = (n1.0 as usize, n1.1 as usize);
		    let n2u = (n2.0 as usize, n2.1 as usize);
		    if grid[n1u.1][n1u.0] == '#' || grid[n2u.1][n2u.0] == '#' {
			check = false;
			break;
		    }
		    if grid[n1u.1][n1u.0] == '.' && grid[n2u.1][n2u.0] == '.' {
			continue;
		    }
		    if grid[n1u.1][n1u.0] == ']' {
			boxes.push(n1);
			boxes.push((n1.0-1, n1.1));
			newtwo.push((n1.0-1, n1.0));
			//two = Some((n1.0-1, n1.0, n1.1 + dir.1));
			
		    }
		    if grid[n1u.1][n1u.0] == '[' {
			boxes.push(n1);
			boxes.push((n1.0+1, n1.1));
			newtwo.push((n1.0, n1.0+1));
			//two = Some((n1.0, n1.0+1, n1.1 + dir.1));
		    }
		    if grid[n2u.1][n2u.0] == '[' {
			boxes.push(n2);
			boxes.push((n2.0+1, n1.1));
			newtwo.push((n2.0, n2.0+1));
			//two = Some((n2.0, n2.0+1, n2.1 + dir.1));
		    }
		}
		// println!("old newtwo {:?}", newtwo);
		// let newtwo: Vec<(i32, i32)> = newtwo.into_iter().unique().collect();
		// let box: Vec<(i32, i32)> = newtwo.into_iter().unique().collect();
		// println!("new newtwo {:?}", newtwo);
		if !check {
		    ok = false;
		    break;
		}
		if newtwo.is_empty() {
		    ok = true;
		    break;
		}
		else {
		    two = Some((newtwo, y + dir.1));
		}
	    }
	    else {
		n = (n.0 + dir.0, n.1 + dir.1);
		if !in_bounds(n, (w, h)) || grid[n.1 as usize][n.0 as usize] == '#' {
		    break;
		}
		if grid[n.1 as usize][n.0 as usize] == '.' {
		    ok = true;
		    break;
		}
		else if grid[n.1 as usize][n.0 as usize] == '['{
		    if dir.0 == 0 {
			boxes.push(n);
			boxes.push((n.0+1, n.1));
			two = Some((vec![(n.0, n.0+1)], n.1+dir.1));
		    }
		    else {
			boxes.push(n);
		    }
		}
		else if grid[n.1 as usize][n.0 as usize] == ']'{
		    if dir.0 == 0 {
			boxes.push(n);
			boxes.push((n.0-1, n.1));
			two = Some((vec![(n.0-1, n.0)], n.1+dir.1));
			//two = Some((n.0-1, n.0, n.1+dir.1));
		    }
		    else {
			boxes.push(n);
		    }
		}
	    }
	}
	if ok {
	    let boxes: Vec<(i32, i32)> = boxes.into_iter().unique().collect();
	    println!("OK!! move code: {} e: {}", code, e);
	    for i in (0..boxes.len()).rev() {
		let b = boxes[i];
		println!("PUSHING: {:?}, dir: {:?}", b, dir);
		grid[(b.1 + dir.1) as usize][(b.0 + dir.0) as usize] = grid[b.1 as usize][b.0 as usize];
		grid[b.1 as usize][b.0 as usize] = '.';
	    }
	    pos = (pos.0 + dir.0, pos.1 + dir.1);
	}
	//if e > 19637 && e < 19642 {
	if false {
	    //println!("ins: {}, After {}: pos: {:?} \n", e, code, pos);
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
