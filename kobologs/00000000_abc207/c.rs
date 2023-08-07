#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::{collections::HashSet, hash::Hash};

fn main() {
    let n: i32 = read!();
    let lr: Vec<(f64, f64)> = (0..n).map(|_| {
        let t: i32 = read!();
        let l: f64 = read!();
        let r: f64 = read!();
        match t {
            1 => (l, r),
            2 => (l, r - 0.1),
            3 => (l + 0.1, r),
            4 => (l + 0.1, r - 0.1),
            _ => panic!()
        }
    }).collect();
    
    let mut ans = 0;
    for (i, &(l, r)) in lr.iter().enumerate() {
        for (j, &(l2, r2)) in lr.iter().enumerate() {
            if i >= j {
                continue;
            }
            if l <= l2 && l2 <= r || l <= r2 && r2 <= r || l2 <= l && r <= r2 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}