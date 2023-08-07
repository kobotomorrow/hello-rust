#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::{collections::HashSet, hash::Hash};

fn main() {
    let n: i32 = read!();
    let mut c: Vec<i64> = (0..n).map(|_| read!()).collect();
    c.sort();
    let mut ans: i64 = 1;
    for i in 0..n {
        ans = ans * (c[i as usize] - i as i64) % 1_000_000_007;
    }
    println!("{}", ans)
}