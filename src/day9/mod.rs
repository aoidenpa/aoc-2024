pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day9.txt"};
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
    let mut s = vec![];
    let mut r = vec![];
    for c in input.chars() {
	if c == '\n' {
	    continue;
	}
	//println!("{}",c);
	let n = (String::from(c)).parse::<i64>().unwrap();
	s.push(n);
    }
    let mut cur = 0;
    for i in 0..s.len() {
	if i % 2 == 0 {
	    for j in 0..s[i] {
		r.push(cur as i64);
	    }
	    cur += 1;
	}
	else {
	    for j in 0..s[i] {
		r.push(-1);
	    }
	}
    }
    // let mut file = true;
    // let mut p1 = 0;
    // let mut p2 = s.len() - 1;
    println!("{:?}", r);
    let mut cur = 0;
    'out: loop {
	if r[cur] != -1 {
	    println!("adding {} * {}", cur as i64, r[cur]);
	    res += cur as i64 * r[cur];
	}
	else {
	    let mut size = 0;
	    while r[cur] != -1 {
		cur += 1;
		size += 1;
	    }
	    
	    
	    let mut a = -1;
	    while a == -1 {
		a = r.pop().unwrap();
		if cur >= r.len() {
		    break 'out;
		}
	    }
	    //let a = r.pop().unwrap();
	    println!("adding {} * {}", cur as i64, a);
	    res += cur as i64 * a;
	}
	cur += 1;
	if cur >= r.len() {
	    break;
	}
    }

    
    res.to_string()
}

pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut parts = vec![];
    let mut id = 0i64;
    for (i, c) in input.chars().enumerate() {
	if c == '\n' {
	    break;
	}
	let n = (String::from(c)).parse::<i64>().unwrap();
	if i % 2 == 0 && n > 0 {
	    parts.push((id, n));
	    id += 1;
	}
	else {
	    if n > 0 {
		parts.push((-1, n));
	    }
	}
    }
    //println!("{:?}", parts);
    let mut i = parts.len() - 1;
    let mut c = 0;
    loop {
	c += 1;
	//println!("{}, {:?}", i, parts);
	// if c > 5 {
	//     break;
	// }
	let cur = parts[i];
	if cur.0 == -1 {
	    i -= 1;
	    continue;
	}
	for j in 0..i {
	    let trys = parts[j];
	    if trys.0 != -1 {
		continue;
	    }
	    if trys.1 >= cur.1 {
		if trys.1 == cur.1 {
		    parts[j].0 = cur.0;
		    parts[i].0 = -1;
		}
		else {
		    parts[i].0 = -1;
		    parts.remove(j);
		    parts.insert(j, cur);
		    parts.insert(j+1, (-1, trys.1-cur.1));
		    i += 1;
		}
		break;
	    }
	}
	if i == 0 {
	    break;
	}
	i -= 1;
    }
    let mut cur = 0;
    for (id, count) in parts {
	if id != -1 {
	    for _ in 0..count {
		res += cur * id;
		cur += 1;
	    }
	}
	else {
	    cur += count;
	}
    }
    //println!("{:?}", parts);
    
    res.to_string()
}
