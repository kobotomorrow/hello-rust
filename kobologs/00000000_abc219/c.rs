#[macro_use] extern crate text_io;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let x: String = read!();
    let mut xx = HashMap::new();
    for (i, c) in x.chars().enumerate() {
        xx.insert(c, i);
    }
    
    let n: usize = read!();
    let mut d = Vec::new();
    for _ in 0..n {
        let s: String = read!();
        let mut cs = Vec::new();
        for c in s.chars() {
            cs.push(xx[&c]);
        }
        d.push((cs, s));
    }
    d.sort();
    for dd in d {
        println!("{}", dd.1);
    }
}
