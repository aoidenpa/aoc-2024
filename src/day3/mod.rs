use regex::Regex;
pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day3.txt"};
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
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input).for_each(|c| res += c.get(1).unwrap().as_str().parse::<i32>().unwrap() * c.get(2).unwrap().as_str().parse::<i32>().unwrap());
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let re = Regex::new(r"(?<mul>mul\((\d+),(\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut flag = true;
    for c in re.captures_iter(&input) {
	if c.name("mul").is_some() {
	    if flag {
		res += c.get(2).unwrap().as_str().parse::<i32>().unwrap() * c.get(3).unwrap().as_str().parse::<i32>().unwrap();
	    }
	}
	else if c.name("do").is_some() {
	    flag = true;
	}
	else if c.name("dont").is_some() {
	    flag = false;
	}
    }
    res.to_string()
}
