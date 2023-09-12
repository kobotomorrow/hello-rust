#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn is_kaibun(s: String) -> bool {
    let ss: String = s.chars().rev().collect();
    s == ss
}

fn main() {
    let s: String = read!();

    let mut hc = 0;
    for c in s.chars() {
        if c == 'a' {
            hc += 1;
        } else {
            break;
        }
    }
    let mut tc = 0;
    for c in s.chars().rev() {
        if c == 'a' {
            tc += 1;
        } else {
            break;
        }
    }
    if hc > tc {
        println!("No");
    } else if hc == s.len() {
        println!("Yes");
    } else if is_kaibun(s[hc..s.len()-tc].to_string()) {
        println!("Yes");
    } else {
        println!("No");
    }
}