#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let mut s = vec![String::from("1")];

    for i in 1..n {
        s.push(format!("{} {} {}", s[i-1], i+1, s[i-1]));
    }

    println!("{}", s[n-1]);
}