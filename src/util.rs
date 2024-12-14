use std::{fmt, str::FromStr};

pub fn nums<T>(s: &str) -> Vec<T>
where
    T: FromStr<Err: fmt::Debug>,
{
    let mut res = vec![];
    let mut cur = String::new();
    let mut has_digit = false;
    for c in s.chars() {
	if c.is_digit(10) {
	    cur.push(c);
	    has_digit = true;
	}
	else if c == '-' {
	    if cur != "" && has_digit {
		res.push(cur.parse::<T>().unwrap());
	    }
	    cur = "-".into();
	    has_digit = false;
	}
	else if cur != "" && has_digit {
	    res.push(cur.parse::<T>().unwrap());
	    cur.clear();
	    has_digit = false;
	}
    }
    if cur != "" && has_digit {
	res.push(cur.parse::<T>().unwrap());
    }
    res
}
