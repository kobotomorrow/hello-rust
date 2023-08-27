#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap, hash::Hash};
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let mut a: Vec<i32> = (0..n).map(|_| read!()).collect();
    a.sort();

    let mut pre = a[0];
    let mut ans = 0;
    for i in 1..n {
        if pre + 1 == a[i] {
            pre = a[i];
            continue;
        }
        ans = pre + 1;
        break;
    }
    println!("{}", ans);
}