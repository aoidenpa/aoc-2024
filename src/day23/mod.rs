use std::collections::{HashMap, HashSet};

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day23.txt"};
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
    let mut gr: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
	let n1 = line.split('-').next().unwrap().to_string();
	let n2 = line.split('-').nth(1).unwrap().to_string();
	let r1 = gr.entry(n1.clone()).or_insert(HashSet::new());
	r1.insert(n2.clone());
	let r2 = gr.entry(n2).or_insert(HashSet::new());
	r2.insert(n1);
    }
    for (a, an) in &gr {
	for ar in an {
	    for b in gr.get(ar).unwrap() {
		if an.contains(b) {
		    if a.starts_with('t') || ar.starts_with('t') || b.starts_with('t') {
			res += 1;
		    }
		}
	    }
	}
    }
    res /= 6;
    res.to_string()
}

// build all cliques. print largest. not fast, around 2 seconds on my machine.
pub fn part2(input: String) -> String {
    let mut res = 0;
    let mut gr = vec![vec![]; 1000];
    let mut map = HashMap::new();
    let mut names = vec![];
    let mut edges = HashSet::new();
    for line in input.lines() {
		let n1 = line.split('-').next().unwrap().to_string();
		let n2 = line.split('-').nth(1).unwrap().to_string();
		if !map.contains_key(&n1) {
			map.insert(n1.clone(), map.len());
			names.push(n1.clone());
		}
		if !map.contains_key(&n2) {
			map.insert(n2.clone(), map.len());
			names.push(n2.clone());
		}
		let &c1 = map.get(&n1).unwrap();
		let &c2 = map.get(&n2).unwrap();
		gr[c1].push(c2);
		gr[c2].push(c1);
		edges.insert((c1, c2));
    }
    let size = map.len();
    let mut comps = vec![];
	for &e in &edges {
		comps.push(vec![e.0, e.1]);
	}
    loop {
		let mut done = true;
		for i in 0..size {
			for c in &mut comps{
				let mut ok = true;
				for k in c.iter() {	
					if !(edges.contains(&(i, *k)) || edges.contains(&(*k, i))) {
						ok = false;
						break;
					}
				}
				if ok {
					c.push(i);
					done = false;
				}
			}
		}
		if done {
			break;
		}
    }
	let mut s = vec![];
	for c in comps {
		if c.len() as i32 > res {
			res = c.len() as i32;
			s = c.clone();
		}
	}
	let mut s = s.into_iter().map(|a| names[a].clone()).collect::<Vec<String>>();
	s.sort();
	let s = s.join(",");
	println!("{}", s);
    res.to_string()
}
