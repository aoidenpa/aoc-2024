use std::collections::HashSet;
pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day5.txt"};
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
    let parts: Vec<&str> = input.split("\n\n").collect();
    let p1 = parts[0].to_string();
    let p2 = parts[1].to_string();
    let mut rules = HashSet::new();
    for line in p1.lines() {
	let mut it = line.split("|");
	let n1 = it.next().unwrap().parse::<i32>().unwrap();
	let n2 = it.next().unwrap().parse::<i32>().unwrap();
	rules.insert((n1, n2));
    }
    for line in p2.lines() {
	let v: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
	let mut ok = true;
	'out: for i in 0..v.len() {
	    for j in i+1..v.len() {
		if rules.contains(&(v[j], v[i])) {
		    ok = false;
		    break 'out;
		}
	    }
	}
	if ok {
	    let mid = v.len() / 2;
	    res += v[mid];
	}
    }
    
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let parts: Vec<&str> = input.split("\n\n").collect();
    let p1 = parts[0].to_string();
    let p2 = parts[1].to_string();
    let mut rules = HashSet::new();
    for line in p1.lines() {
	let mut it = line.split("|");
	let n1 = it.next().unwrap().parse::<i32>().unwrap();
	let n2 = it.next().unwrap().parse::<i32>().unwrap();
	rules.insert((n1, n2));
    }
    for line in p2.lines() {
	let mut v: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
	let mut fixed = false;
	loop {
	    let mut done = true;
	    for i in 0..v.len() {
		for j in i+1..v.len() {
		    if rules.contains(&(v[j], v[i])) {
			v.swap(i, j);
			done = false;
			fixed = true;
		    }
		}
	    }
	    if done {
		if fixed {
		    let mid = v.len() / 2;
		    res += v[mid];
		}
		break;
	    }
	}
    }
    
    res.to_string()
}
