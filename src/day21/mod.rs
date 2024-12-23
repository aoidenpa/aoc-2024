use std::collections::HashMap;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day21.txt"};
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
    let num_map = HashMap::from([
	('0', (1, 0)),
	('1', (0, 1)),
	('2', (1, 1)),
	('3', (2, 1)),
	('4', (0, 2)),
	('5', (1, 2)),
	('6', (2, 2)),
	('7', (0, 3)),
	('8', (1, 3)),
	('9', (2, 3)),
	('A', (2, 0)),
    ]);
    let dir_map = HashMap::from([
	('>', (2, 0)),
	('<', (0, 0)),
	('^', (1, 1)),
	('v', (1, 0)),
	('A', (2, 1)),
    ]);

    let mut rec = HashMap::new();
    for line in input.lines() {
	let mut loc = 0;
	let n = (&line[..line.len()-1]).parse::<i64>().unwrap();
	let v = line.chars().collect::<Vec<char>>();
	let cur = (2, 0);
	let min = usize::MAX;
	let mut start = 'A';
	for a in v {
	    loc += dp(start, a, 1, 0, &num_map, &dir_map, &mut rec, 2);
	    start = a;
	}
	res += n * loc;
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let num_map = HashMap::from([
	('0', (1, 0)),
	('1', (0, 1)),
	('2', (1, 1)),
	('3', (2, 1)),
	('4', (0, 2)),
	('5', (1, 2)),
	('6', (2, 2)),
	('7', (0, 3)),
	('8', (1, 3)),
	('9', (2, 3)),
	('A', (2, 0)),
    ]);
    let dir_map = HashMap::from([
	('>', (2, 0)),
	('<', (0, 0)),
	('^', (1, 1)),
	('v', (1, 0)),
	('A', (2, 1)),
    ]);

    let mut rec = HashMap::new();
    for line in input.lines() {
	let mut loc = 0;
	let n = (&line[..line.len()-1]).parse::<i64>().unwrap();
	let v = line.chars().collect::<Vec<char>>();
	let cur = (2, 0);
	let min = usize::MAX;
	let mut start = 'A';
	for a in v {
	    loc += dp(start, a, 1, 0, &num_map, &dir_map, &mut rec, 25);
	    start = a;
	}
	res += n * loc;
    }
    res.to_string()
}

// gave me a headache trying to wrap my head around this dp. looks horrible. needs refining.
fn dp(start: char, to: char, press: i64, dep: i64, num_map: &HashMap<char, (i64, i64)>, dir_map: &HashMap<char, (i64, i64)>,
      rec: &mut HashMap<(char, char, i64, i64), i64>, end: i64) -> i64 {
    if let Some(r) = rec.get(&(start, to, press, dep)) {
	return *r;
    }
    let res;
    let nullp = if dep == 0 {(0, 0)} else {(0, 1)};
    let &start_pos = if dep == 0 {num_map.get(&start).unwrap()} else {dir_map.get(&start).unwrap()};
    let &to_pos = if dep == 0 {num_map.get(&to).unwrap()} else {dir_map.get(&to).unwrap()};
    let dx = to_pos.0 - start_pos.0;
    let dy = to_pos.1 - start_pos.1;
    let xc = if dx > 0 {'>'} else {'<'};
    let yc = if dy > 0 {'^'} else {'v'};
    if press == 0 {
	res = 0;
    }
    //the last depth, just raw code.
    else if dep == end {
	res = dx.abs() + dy.abs() + press;
    }
    // can't move in x dir cause we hit empty pos.
    else if (to_pos.0, start_pos.1) == nullp {
	res = dp('A', yc, dy.abs(), dep+1, num_map, dir_map, rec, end) + dp(yc, xc, dx.abs(), dep+1, num_map, dir_map, rec, end) + dp(xc, 'A', press, dep+1, num_map, dir_map, rec, end);
    }
    // can't move in y dir cause we hit empty pos.
    else if (start_pos.0, to_pos.1) == nullp {
	res = dp('A', xc, dx.abs(), dep+1, num_map, dir_map, rec, end) + dp(xc, yc, dy.abs(), dep+1, num_map, dir_map, rec, end) + dp(yc, 'A', press, dep+1, num_map, dir_map, rec, end);
    }
    else {
	if dx == 0 {
	    res = dp('A', yc, dy.abs(), dep+1, num_map, dir_map, rec, end) + dp(yc, 'A', press, dep+1, num_map, dir_map, rec, end);
	}
	else if dy == 0 {
	    res = dp('A', xc, dx.abs(), dep+1, num_map, dir_map, rec, end) + dp(xc, 'A', press, dep+1, num_map, dir_map, rec, end);
	}
	// we got 2 ways to go, vertical first, then horizontal, and vice versa. try both and return min.
	// vert first case: start from A, go to yc(y character), '<' or '>', and press dy times. then, go to xc and press dx times.
	// then go to 'A' and press "press" times
	else {
	    let a1 = dp('A', yc, dy.abs(), dep+1, num_map, dir_map, rec, end) + dp(yc, xc, dx.abs(), dep+1, num_map, dir_map, rec, end) + dp(xc, 'A', press, dep+1, num_map, dir_map, rec, end);
	    let a2 = dp('A', xc, dx.abs(), dep+1, num_map, dir_map, rec, end) + dp(xc, yc, dy.abs(), dep+1, num_map, dir_map, rec, end) + dp(yc, 'A', press, dep+1, num_map, dir_map, rec, end);
	    res =  a1.min(a2);
	}
    }
    rec.insert((start, to, press, dep), res);
    return res;
    
}
