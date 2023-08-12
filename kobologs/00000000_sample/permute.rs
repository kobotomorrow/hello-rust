#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque, HashSet};

fn permute(s: Vec<char>, l: usize, r: usize, result: &mut HashSet<String>) {
    if l == r {
        result.insert(s.iter().collect());
    } else {
        for i in l..=r {
            let mut s = s.clone();
            s.swap(l, i);
            permute(s, l + 1, r, result);
        }
    }
}

fn main() {
    let s: &str = "abc";
    let mut result = HashSet::new();
    permute(s.chars().collect(), 0, s.len() - 1, &mut result);
    let result: Vec<String> = result.into_iter().collect();
    println!("{:?}", result);
}
