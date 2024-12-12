use std::collections::HashMap;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day11.txt"};
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
    let nums = input.trim().split(" ").map(|n| { n.parse::<i64>().unwrap()}).collect::<Vec<i64>>();
    let count = 25;
    let mut m =  HashMap::new();
    for n in nums {
	res += dp(n, count, &mut m);
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let nums = input.trim().split(" ").map(|n| { n.parse::<i64>().unwrap()}).collect::<Vec<i64>>();
    let count = 75;
    let mut m =  HashMap::new();
    for n in nums {
	res += dp(n, count, &mut m);
    }
    res.to_string()
}

fn divide(n: i64) -> (i32, i64, i64) {
    let mut tn = n;
    let mut s = 0;
    let mut ten = 1;
    while tn > 0 {
	tn /= 10;
	s += 1;
	if s % 2 == 0 {
	    ten *= 10
	}
    }
    return (s, n / ten, n % ten);
}

fn dp(n: i64, len: i32, m: &mut HashMap<(i64, i32), i64>) -> i64 {
    let res;
    if len == 0 {
	return 1;
    }
    if let Some(r) = m.get(&(n, len)) {
	return *r;
    }
    else {
	if n == 0 {
	    res = dp(1, len-1, m);
	}
	else {
	    let (size, left, right) = divide(n);
	    if size % 2  == 0 {
		res = dp(left, len-1, m) + dp(right, len-1, m);
	    }
	    else {
		res = dp(n*2024, len-1, m);
	    }
	}
    }
    m.insert((n, len), res);
    return res;
}

