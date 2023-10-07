#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let s: String = read!();
    for (i, c) in s.chars().enumerate() {
        if (i+1) % 2 == 0 && c != '0' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}