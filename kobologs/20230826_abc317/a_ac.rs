#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap, hash::Hash};
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: i32 = read!();
    let h: i32 = read!();
    let x: i32 = read!();
    let p: Vec<i32> = (0..n).map(|_| read!()).collect();

    for i in 0..n {
        if p[i as usize] + h >= x {
            println!("{}", i + 1);
            return;
        }
    }
}