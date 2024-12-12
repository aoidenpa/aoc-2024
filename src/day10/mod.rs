use itertools::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day10.txt"};
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
	grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>());
    }
    let w = grid[0].len();
    let h = grid.len();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == 0 {
		let mut q = vec![];
		let mut set = HashSet::new();
		q.push((i, j, 0));
		while !q.is_empty() {
		    let (x, y, val) = q.pop().unwrap();
		    if val == 9 && !set.contains(&(x, y)) {
			res += 1;
			set.insert((x, y));
		    }
		    else {
			for dir in dirs {
			    let n = (x as i32 + dir.0 as i32, y as i32 + dir.1 as i32);
			    if n.0 >= 0 && n.0 < w as i32 && n.1 >= 0 && n.1 < h as i32 {
				if grid[n.1 as usize][n.0 as usize] == val + 1 {
				    q.push((n.0 as usize, n.1 as usize, val+1));
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

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid = vec![];
    for line in input.lines() {
	grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>());
    }
    let w = grid[0].len();
    let h = grid.len();
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for i in 0..w {
	for j in 0..h {
	    if grid[j][i] == 0 {
		let mut q = vec![];
		q.push((i, j, 0));
		while !q.is_empty() {
		    let (x, y, val) = q.pop().unwrap();
		    if val == 9 {
			res += 1;
		    }
		    else {
			for dir in dirs {
			    let n = (x as i32 + dir.0 as i32, y as i32 + dir.1 as i32);
			    if n.0 >= 0 && n.0 < w as i32 && n.1 >= 0 && n.1 < h as i32 {
				if grid[n.1 as usize][n.0 as usize] == val + 1 {
				    q.push((n.0 as usize, n.1 as usize, val+1));
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
