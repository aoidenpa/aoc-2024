use std::collections::HashSet;

use itertools::iproduct;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day8.txt"};
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
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut set = HashSet::new();
    for (i1, j1) in iproduct!(0..w, 0..h) {
	for (i2, j2) in iproduct!(0..w, j1..h) {
	    if i1 == i2 && j1 == j2 {
		continue;
	    }
	    let a = grid[j1 as usize][i1 as usize];
	    let b = grid[j2 as usize][i2 as usize];
	    if a == b && a != '.' {
		let d = (i2 - i1, j2 - j1);
		let a1 = (i1 - d.0, j1  - d.1);
		let a2 = (i2  + d.0, j2  + d.1);
		if a1.0 >= 0 && a1.0 < w  && a1.1 >= 0 && a1.1 < h  {
		    set.insert(a1);
		}
		if a2.0 >= 0 && a2.0 < w  && a2.1 >= 0 && a2.1 < h  {
		    set.insert(a2);
		}
	    }
	}
    }
    res = set.len();
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid = vec![];
    for line in input.lines() {
	grid.push(line.chars().collect::<Vec<char>>());
    }
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut set = HashSet::new();
    for (i1, j1) in iproduct!(0..w, 0..h) {
	for (i2, j2) in iproduct!(0..w, j1..h) {
	    if i1 == i2 && j1 == j2 {
		continue;
	    }
	    let a = grid[j1 as usize][i1 as usize];
	    let b = grid[j2 as usize][i2 as usize];
	    if a == b && a != '.' {
		let d = ((i2 - i1), (j2 - j1));
		let mut a1 = (i1 , j1);
		let mut a2 = (i2 , j2);
		loop {
		    let mut done = true;
		    if a1.0 >= 0 && a1.0 < w  && a1.1 >= 0 && a1.1 < h  {
			set.insert(a1);
			a1 = (a1.0 - d.0, a1.1 - d.1);
			done = false;
		    }
		    if a2.0 >= 0 && a2.0 < w  && a2.1 >= 0 && a2.1 < h {
			set.insert(a2);
			a2 = (a2.0 + d.0, a2.1 + d.1);
			done = false;
		    }
		    if done {
			break;
		    }
		}
	    }
	}
    }
    res = set.len();
    res.to_string()
}
