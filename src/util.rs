use std::{fmt, str::FromStr};

pub fn nums<T>(s: &str) -> Vec<T>
where
    T: FromStr<Err: fmt::Debug>,
{
    let mut res = vec![];
    let mut cur = String::new();
    for c in s.chars() {
	if c.is_digit(10) {
	    cur.push(c);
	}
	else if c == '-' {
	    if cur != "" {
		res.push(cur.parse::<T>().unwrap());
		cur.clear();
	    }
	    cur.push(c);
	}
	else if cur != "" {
	    res.push(cur.parse::<T>().unwrap());
	    cur.clear();
	}
    }
    if cur != "" {
	res.push(cur.parse::<T>().unwrap());
    }
    res
}
