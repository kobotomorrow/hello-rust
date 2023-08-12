#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn cast(x: usize, c: char) -> char {
    let mut r = c as u8;
    if x == 2 {
        if c.is_uppercase() {
            r += 32;
        }
    } else if x == 3 {
        if c.is_lowercase() {
            r -= 32;
        }
    }
    return r as char;
}

fn main() {
    let _: usize = read!();
    let s: String = read!();
    let mut s = s.chars().collect::<Vec<char>>();
    let q: usize = read!();
    let mut lu = 0;
    let mut lu_i = 0;
    let mut v = vec![];

    for i in 0..q {
        let a: usize = read!();
        let b: usize = read!();
        let c: char = read!();

        if a == 2 || a == 3 {
            lu = a;
            lu_i = i;
        } else {
            v.push((b, c, i));
        }
    }

    s = s.iter().map(|c| cast(lu, *c)).collect::<Vec<char>>();
    
    for vv in v {
        if vv.2 < lu_i {
            s[vv.0 - 1] = cast(lu, vv.1);
        } else {
            s[vv.0 - 1] = vv.1;
        }
    }

    for c in s {
        print!("{}", c);
    }
    println!();
}
