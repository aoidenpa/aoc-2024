use std::collections::{HashMap, HashSet};

pub fn run(part: u32, input: String) -> String {
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
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect());
    }
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut start = (0, 0);
    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '^' {
		start = (i as i32, j as i32);
	    }
	}
    }
    // let mut cur = start;
    // let mut dir = (0, -1);
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    //let mut set = HashSet::new();
    let mut map = HashMap::new();
    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '#' {
		continue;
	    }
	    //let mut cur = (i, j);
	    for dir in dirs {
		let mut cur = (i, j);
		loop {
		    let next = (cur.0+dir.0, cur.1+dir.1);
		    if next.0 < 0 || next.0 == w as i32 || next.1 < 0 || next.1 == h as i32 {
			break;
		    }
		    if grid[next.1 as usize][next.0 as usize] == '#' {
			map.insert(((i, j), dir), cur);
			break;
		    }
		    cur = next;
		}
	    }
	}
    }
    //println!("{:?}", map);
    let mut cur = start;
    let mut dir = (0, -1);
    let mut set = HashSet::new();
    set.insert(cur);
    loop {
	if let Some(&next) = map.get(&(cur, dir)) {
	    let mut s = cur;
	    let mut e = next;
	    loop {
		//println!("{:?}, {:?}", s, e);
		s = (s.0+dir.0, s.1+dir.1);
		set.insert(s);
		if s == e {
		    break;
		}
	    }
	    cur = next;
	    dir = (-dir.1, dir.0);
	}
	else {
	    let mut s = cur;
	    loop {
		s = (s.0+dir.0, s.1+dir.1);
		if s.0 < 0 || s.0 == w || s.1 < 0 || s.1 == h {
		    break;
		}
		set.insert(s);
	    }
	    break;
	}
    }
    res = set.len();
    res.to_string()
}
fn getset(input: String) -> HashSet<(i32, i32)> {
    let mut res = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect());
    }
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut start = (0, 0);
    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '^' {
		start = (i as i32, j as i32);
	    }
	}
    }
    // let mut cur = start;
    // let mut dir = (0, -1);
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    //let mut set = HashSet::new();
    let mut map = HashMap::new();
    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '#' {
		continue;
	    }
	    //let mut cur = (i, j);
	    for dir in dirs {
		let mut cur = (i, j);
		loop {
		    let next = (cur.0+dir.0, cur.1+dir.1);
		    if let Some(&res) = map.get(&(next, dir)) {
			map.insert(((i, j), dir), res);
			break;
		    }
		    if next.0 < 0 || next.0 == w as i32 || next.1 < 0 || next.1 == h as i32 {
			break;
		    }
		    if grid[next.1 as usize][next.0 as usize] == '#' {
			map.insert(((i, j), dir), cur);
			break;
		    }
		    cur = next;
		}
	    }
	}
    }
    let mut cur = start;
    let mut dir = (0, -1);
    let mut set = HashSet::new();
    set.insert(cur);
    loop {
	if let Some(&next) = map.get(&(cur, dir)) {
	    let mut s = cur;
	    let mut e = next;
	    loop {
		//println!("{:?}, {:?}", s, e);
		s = (s.0+dir.0, s.1+dir.1);
		set.insert(s);
		if s == e {
		    break;
		}
	    }
	    cur = next;
	    dir = (-dir.1, dir.0);
	}
	else {
	    let mut s = cur;
	    loop {
		s = (s.0+dir.0, s.1+dir.1);
		if s.0 < 0 || s.0 == w || s.1 < 0 || s.1 == h {
		    break;
		}
		set.insert(s);
	    }
	    break;
	}
    }
    return set;

}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect());
    }
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut start = (0, 0);
    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '^' {
		start = (i as i32, j as i32);
	    }
	}
    }
    // let mut cur = start;
    // let mut dir = (0, -1);
    
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    //let mut set = HashSet::new();
    let mut map = HashMap::new();
    // let mut rows = vec![];
    // let mut cols = vec![];

    for i in 0..w {
	for j in 0..h {
	    if grid[j as usize][i as usize] == '#' {
		continue;
	    }
	    //let mut cur = (i, j);
	    for dir in dirs {
		let mut cur = (i, j);
		loop {
		    let next = (cur.0+dir.0, cur.1+dir.1);
		    if next.0 < 0 || next.0 == w as i32 || next.1 < 0 || next.1 == h as i32 {
			break;
		    }
		    if grid[next.1 as usize][next.0 as usize] == '#' {
			map.insert(((i, j), dir), cur);
			break;
		    }
		    cur = next;
		}
	    }
	}
    }
    // let mut cur = start;
    // let mut dir = (0, -1);
    // let mut set = HashSet::new();
    // set.insert((cur, dir));
    let aset = getset(input);
    for (i, j) in aset {
	if grid[j as usize][i as usize] == '#' || grid[j as usize][i as usize] == '^' {
	    continue;
	}
	let mut cur = start;
	let mut dir = (0, -1);
	let mut set = HashSet::new();
	//let mut nmap = map.clone();
	//let mut nmap = HashMap::new();
	let mut originals = HashMap::new();
	for dir in dirs {
	    //nmap.remove(&((i, j), (dir)));
	    let mut n = (i+dir.0, j+dir.1);
	    let m = n;
	    if n.0 < 0 || n.0 == w || n.1 < 0 || n.1 == h {
		continue;
	    }
	    loop {
		if n.0 < 0 || n.0 == w || n.1 < 0 || n.1 == h {
		    break;
		}
		if grid[n.1 as usize][n.0 as usize] == '#' {
		    break;
		}
		let or_val = map.remove(&((n.0, n.1), (-dir.0, -dir.1)));
		originals.insert(((n.0, n.1), (-dir.0, -dir.1)), or_val);
		map.insert(((n.0, n.1), (-dir.0, -dir.1)), m);
		n = (n.0+dir.0, n.1+dir.1);
	    }
	}

	loop {
	    if set.contains(&(cur, dir)) {
		res += 1;
		break;
	    }
	    set.insert((cur, dir));
	    if let Some(&next) = map.get(&(cur, dir)) {
		cur = next;
		dir = (-dir.1, dir.0);
	    }
	    else {
		break;
	    }
	}
	for (k, v) in originals {
	    if let Some(or) = v {
		map.insert(k, or);
	    }
	    else {
		map.remove(&k);
	    }
	}
	
	grid[j as usize][i as usize] = '.';
    }
    res.to_string()
}
