pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day2.txt"};
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
	let nums: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
	let dir = nums[1] - nums[0];
	let mut ok = true;
	for i in 1..nums.len() {
	    let d = nums[i] - nums[i - 1];
	    if d.signum() != dir.signum() || d.abs() > 3 {
		ok = false;
		break;
	    }
	}
	if ok {
	    res += 1;
	}
    }
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    for line in input.lines() {
	let nums: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
	let dir = nums[1] - nums[0];
	for i in 0..nums.len() {
	    if check(&nums, i) {
		res += 1;
		break;
	    }
	}
    }
    res.to_string()
}
fn check(nums: &Vec<i32>, skip: usize) -> bool {
    let mut dir = None;
    let mut ok = true;
    for i in 0..nums.len() {
	let mut next = i+1;
	if i == skip || next == nums.len() {
	    continue;
	}
	if next == skip {
	    next += 1;
	}
	if next == nums.len() {
	    continue;
	}
	if dir.is_none() {
	    dir = Some(nums[next] - nums[i]);
	}
	let d = nums[next] - nums[i];
	if d.signum() != dir.unwrap().signum() || d.abs() > 3 {
	    ok = false;
	    break;
	}
    }
    ok
}
