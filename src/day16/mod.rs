use std::collections::{HashMap, HashSet, VecDeque};
use itertools::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day16.txt"};
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
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == 'S' {
		start = (i, j);
		grid[j][i] = '.';
	    }
	    if grid[j][i] == 'E' {
		end = (i, j);
		grid[j][i] = '.';
	    }
	}
    }
    let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut q = VecDeque::new();
    q.push_back((start, 0, (1, 0)));
    let mut map = HashMap::new();
    let mut best = i32::MAX;
    let mut scores = vec![];
    //BFS
    while !q.is_empty() {
	let (cur, score, d) = q.pop_front().unwrap();
	if let Some(res) = map.get(&(cur, d)) {
	    if score < *res {
		map.insert((cur, d), score);
	    }
	    else {
		continue;
	    }
	}
	else {
	    map.insert((cur, d), score);
	}
	if cur == end {
	    best = best.min(score);
	    scores.push(score);
	    continue;
	}
	if score > best {
	    continue;
	}
	let n = (cur.0 as i32 + d.0, cur.1 as i32 + d.1);
	if grid[n.1  as usize][n.0 as usize] == '.' {
	    q.push_back(((n.0 as usize, n.1 as usize), score+1, d));
	}
	let d2 = (-d.1, d.0);
	let d3 = (d.1, -d.0);
	q.push_back((cur, score+1000, d2));
	q.push_back((cur, score+1000, d3));
    }
    scores.sort();
    res = scores[0];
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

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == 'S' {
		start = (i, j);
		grid[j][i] = '.';
	    }
	    if grid[j][i] == 'E' {
		end = (i, j);
		grid[j][i] = '.';
	    }
	}
    }
    let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut maps = vec![];
    let mut best = i32::MAX;
    for k in 0..2 {
	let mut q = VecDeque::new();
	let mut set = HashSet::new();
	set.insert(start);
	if k == 1 {
	    let t = end;
	    end = start;
	    start = t;
	    set.insert(start);
	    q.push_back((start, 0, (1, 0)));
	    q.push_back((start, 0, (-1, 0)));
	    q.push_back((start, 0, (0, 1)));
	    q.push_back((start, 0, (0, -1)));
	}
	else {
	    set.insert(start);
	    q.push_back((start, 0, (1, 0)));
	}
	let mut map = HashMap::new();
	//BFS with pathset
	while !q.is_empty() {
	    let (cur, score, d) = q.pop_front().unwrap();
	    if let Some(res) = map.get(&(cur, d)) {
		if score <= *res {
		    map.insert((cur, d), score);
		}
		else {
		    continue;
		}
	    }
	    else {
		map.insert((cur, d), score);
	    }
	    if cur == end {
		if k == 0 {
		    best = best.min(score);
		}
		continue;
	    }
	    let n = (cur.0 as i32 + d.0 as i32, cur.1 as i32 + d.1 as i32);
	    if grid[n.1  as usize][n.0 as usize] == '.' {
		q.push_back(((n.0 as usize, n.1 as usize), score+1, d));
	    }
	    let d2 = (-d.1, d.0);
	    let d3 = (d.1, -d.0);
	    q.push_back((cur, score+1000, d2));
	    q.push_back((cur, score+1000, d3));
	}
	maps.push(map);
    }
    let mut s = HashSet::new();
    for i in 0..w {
	for j in 0..h {
	    for d in dirs {
		if let Some(r1) = maps[0].get(&((i, j), d)) {
		    if let Some(r2) = maps[1].get(&((i, j), (-d.0, -d.1))) {
			if *r1 + *r2 == best {
			    s.insert((i, j));
			}
		    }
		}
	    }
	}
    }
    println!("aa {:?}", maps[0].get(&((1, 11), (0, -1))));
    println!("bb {:?}", maps[1].get(&((1, 11), (0, 1))));
    res = s.len();
    
    
    // scores.sort_by(|(a1, b1), (a2, b2)| a1.cmp(a2));
    // res = scores.into_iter().
    // 	filter(|(score, set)| *score == best).
    // 	fold(HashSet::new(), |acc, x|
    // 	     acc.union(&x.1).map(|a| *a).collect::<HashSet<(usize, usize)>>())
    // 	.len();
    res.to_string()
}
