use std::collections::HashMap;
pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day1.txt"};
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
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
	let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	left.push(nums[0]);
	right.push(nums[1]);
    }
    left.sort();
    right.sort();
    res = left.into_iter().zip(right.into_iter()).map(|(l, r)| (l-r).abs()).sum::<i32>();
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut left = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
	let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	left.push(nums[0]);
	let e = map.entry(nums[1]).or_insert(0);
	*e += 1;
    }
    res = left.into_iter().map(|l| l * map.get(&l).unwrap_or(&0)).sum::<i32>();
    res.to_string()
}
