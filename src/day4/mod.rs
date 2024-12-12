pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day4.txt"};
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
    let mut grid : Vec<Vec<char>> = vec![];
    for line in input.lines() {
	let c = line.chars().collect::<Vec<char>>();
	grid.push(c);
    }
    let w = grid[0].len();
    let h = grid.len();
    for i in 0..w {
	for j in 0..h {
	    for di in -1..=1 {
		for dj in -1..=1 {
		    if di == 0 && dj == 0 {
			continue;
		    }
		    if check(&grid, (i, j), (di, dj)) {
			res += 1;
		    }
		}
	    }
	}
    }
    res.to_string()
}

fn check(grid: &Vec<Vec<char>>, mut s: (usize, usize), dir: (i32, i32)) -> bool {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let end = (s.0 as i32 + 3*dir.0, s.1 as i32 + 3*dir.1);
    let xmas: Vec<char> = "XMAS".chars().collect();
    if end.0 >=0 && end.0 < w && end.1 >= 0 && end.1 < h {
	for i in 0..4 {
	    if grid[s.1][s.0] != xmas[i] {
		return false;
	    }
	    s.0 = (s.0 as i32 + dir.0) as usize;
	    s.1 = (s.1 as i32 + dir.1) as usize;
	}
	return true;
    }
    else {
	return false;
    }
}
fn checkmas(grid: &Vec<Vec<char>>, s: (usize, usize)) -> bool {
    if grid[s.1+1][s.0+1] != 'A' {
	return false;
    }
    let s1: String = vec![grid[s.1][s.0], grid[s.1+2][s.0+2]].into_iter().collect();
    let s2: String = vec![grid[s.1+2][s.0], grid[s.1][s.0+2]].into_iter().collect();
    if (s1 == "SM" || s1 == "MS") && (s2 == "SM" || s2 == "MS") {
	return true;
    }
    return false;
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut grid : Vec<Vec<char>> = vec![];
    for line in input.lines() {
	let c = line.chars().collect::<Vec<char>>();
	grid.push(c);
    }
    let w = grid[0].len();
    let h = grid.len();
    for i in 0..w-2 {
	for j in 0..h-2 {
	    if checkmas(&grid, (i, j)) {
		res += 1;
	    }
	}
    }
    res.to_string()
}
