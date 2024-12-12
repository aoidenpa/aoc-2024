use std::collections::HashSet;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day7.txt"};
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
    for line in input.lines() {
	let mut it = line.split(":");

	let target = it.next().unwrap().parse::<i64>().unwrap();
	let rest = it.next().unwrap();
	let mut nums: Vec<i64> = rest.trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
	let size = nums.len() - 1;
	nums.reverse();
	let mut found = HashSet::new();
	rec(target, &mut nums, &mut res, &mut found, 1);
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    for line in input.lines() {
	let mut it = line.split(":");

	let target = it.next().unwrap().parse::<i64>().unwrap();
	let rest = it.next().unwrap();
	let mut nums: Vec<i64> = rest.trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
	let size = nums.len() - 1;
	nums.reverse();
	let mut found = HashSet::new();
	rec(target, &mut nums, &mut res, &mut found, 2);
    }
    res.to_string()
}

fn rec(target: i64, nums: &mut Vec<i64>, res: &mut i64, found: &mut HashSet<i64>, part: i32) {
    if nums.len() == 1 && nums[0] == target {
	if !found.contains(&target) {
	    *res += target;
	}
	found.insert(target);
	return;
    }
    else if nums.len() == 1 && nums[0] != target {
	return;
    }
    else if nums[0] >= target {
	return;
    }
    else {
	let mut n1 = nums.pop().unwrap();
	let n2 = nums.pop().unwrap();
	let c1 = n1+n2;
	let c2 = n1*n2;
	let mut tn2 = n2;
	let c3 = loop {
	    n1 *= 10;
	    tn2 /= 10;
	    if tn2 == 0 {
		break n1+n2;
	    }
	};
	let mut cs = vec![c1,c2];
	if part == 2 {cs.push(c3);}
	for c in cs {
	    nums.push(c);
	    rec(target, nums, res, found, part);
	    nums.pop();
	}
	nums.push(n2);
	nums.push(n1);
    }
}







