use std::{char::from_digit, collections::{HashMap, HashSet}};
use itertools::*;

pub fn run(part: u32, test: bool) -> String {
    let path = if test {"test_input"} else {"input/day24.txt"};
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
    let p1 = input.split("\n\n").nth(0).unwrap();
    let p2 = input.split("\n\n").nth(1).unwrap();
    let mut v = vec![];
    let mut map = HashMap::new();
    for line in p2.lines() {
        let it = line.split(" ").collect::<Vec<&str>>();
        v.push((it[0].to_string(), it[1].to_string(), it[2].to_string(), it[4].to_string()));
        map.insert(it[0].to_string(), None);
        map.insert(it[2].to_string(), None);
        map.insert(it[4].to_string(), None);
    }
    for line in p1.lines() {
        let it = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
        map.insert(it[0].to_string(), Some(it[1].parse::<i32>().unwrap()));
    }
    loop {
        let mut done = true;
        for (n1, op, n2, to) in &v {
            if map[n1].is_some() && map[n2].is_some() && map[to].is_none() {
                println!("{:?}", (n1, op, n2, to));
                let m1 = map.get(n1).unwrap();
                let m2 = map.get(n2).unwrap();
                if false {
                    if op == "xor" && !n1.starts_with("x") && !n1.starts_with("y") && !n2.starts_with("x") && 
                    !n2.starts_with("y") &&
                    !to.starts_with("z") {
                        println!("nope: {:?}", (n1, op, n2, to));
                    }
                    if op != "xor" && to.starts_with("z") {
                        println!("nope2: {:?}", (n1, op, n2, to));   
                    }
                }

                match op.as_str() {
                    "AND" => *map.get_mut(to).unwrap() = Some(m1.unwrap() & m2.unwrap()),
                    "OR" => *map.get_mut(to).unwrap() = Some(m1.unwrap() | m2.unwrap()), 
                    "XOR" => *map.get_mut(to).unwrap() = Some(m1.unwrap() ^ m2.unwrap()), 
                    _ => panic!(),
                }
            }
        }
        for (k, v) in &map {
            if k.starts_with("z") && v.is_none() {
                done = false;
                break;  
            }
        }
        if done {
            break;
        }
    }               
    let mut r = vec![];
    for (k, v) in &map {
        if k.starts_with("z") && v.is_some() {
            r.push((k.clone(), v.unwrap()));
        }
    }
    r.sort();
    r.reverse();
    let s = r.iter().map(|(_, n)| from_digit(*n as u32, 10).unwrap()).collect::<String>();

    res = i64::from_str_radix(&s, 2).unwrap();


    res.to_string()
}

// prints 3 pairs of nodes that are in wrong place. all XOR operations should be stored in z's.
// i found last pair by editing x and y to see if they add. found the first place the result was different and checked by hand.
pub fn part2(input: String) -> String {
    let mut res = 0;
    let p1 = input.split("\n\n").nth(0).unwrap();
    let p2 = input.split("\n\n").nth(1).unwrap();
    let mut v = vec![];
    let mut map = HashMap::new();
    for line in p2.lines() {
        let it = line.split(" ").collect::<Vec<&str>>();
        v.push((it[0].to_string(), it[1].to_string(), it[2].to_string(), it[4].to_string()));
        map.insert(it[0].to_string(), None);
        map.insert(it[2].to_string(), None);
        map.insert(it[4].to_string(), None);
    }
    for line in p1.lines() {
        let it = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
        let m = it[1].parse::<i32>().unwrap();
        map.insert(it[0].to_string(), Some(m));
    }
    loop {
        let mut done = true;
        for (n1, op, n2, to) in &v {
            if map[n1].is_some() && map[n2].is_some() && map[to].is_none() {
                let m1 = map.get(n1).unwrap();
                let m2 = map.get(n2).unwrap();

                if op == "XOR" && !n1.starts_with("x") && !n1.starts_with("y") && !n2.starts_with("x") && 
                !n2.starts_with("y") &&
                !to.starts_with("z") {
                    println!("nope: {:?}", (n1, op, n2, to));
                }
                if op != "XOR" && to.starts_with("z") {
                    println!("nope: {:?}", (n1, op, n2, to));   
                }

                match op.as_str() {
                    "AND" => *map.get_mut(to).unwrap() = Some(m1.unwrap() & m2.unwrap()),
                    "OR" => *map.get_mut(to).unwrap() = Some(m1.unwrap() | m2.unwrap()), 
                    "XOR" => *map.get_mut(to).unwrap() = Some(m1.unwrap() ^ m2.unwrap()), 
                    _ => panic!(),
                }
            }
        }
        for (k, v) in &map {
            if k.starts_with("z") && v.is_none() {
                done = false;
                break;  
            }
        }
        if done {
            break;
        }
    }               
    let mut r = vec![vec![]; 3];
    for (k, v) in &map {
        if k.starts_with("x") && v.is_some() {
            r[0].push((k.clone(), v.unwrap()));
        }
    }
    for (k, v) in &map {
        if k.starts_with("y") && v.is_some() {
            r[1].push((k.clone(), v.unwrap()));
        }
    }
    for (k, v) in &map {
        if k.starts_with("z") && v.is_some() {
            r[2].push((k.clone(), v.unwrap()));
        }
    }
    for i in 0..3 {
        r[i].sort();
        r[i].reverse();
    }
    let s1 = r[0].iter().map(|(_, n)| from_digit(*n as u32, 10).unwrap()).collect::<String>();
    let s2 = r[1].iter().map(|(_, n)| from_digit(*n as u32, 10).unwrap()).collect::<String>();
    let s3 = r[2].iter().map(|(_, n)| from_digit(*n as u32, 10).unwrap()).collect::<String>();
    let n1 = i64::from_str_radix(&s1, 2).unwrap();
    let n2 = i64::from_str_radix(&s2, 2).unwrap();
    let sum = n1+n2;

    println!("x:  {}", s1);
    println!("y:  {}", s2);
    println!("actual: {:b}", sum);
    println!("z:      {}", s3);

    res.to_string()
}
