#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::{collections::HashSet, hash::Hash};

fn main() {
    let n: i64 = read!();
    let k: i64 = read!();
    let m = k/n;
    let mut t: Vec<(i64, i64, i64)> = (0..n).map(|i| {
        let a: i64 = read!();
        (a, i, m)
    }).collect();
    
    t.sort();
    let kk:usize = (k % n) as usize;
    for i in 0..kk {
        t[i].2 += 1;
    }
    t.sort_by(|a: &(i64, i64, i64), b| a.1.cmp(&b.1));
    for i in 0..n {
        println!("{}", t[i as usize].2);
    }
}