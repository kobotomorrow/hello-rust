#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;


fn main() {
    let k: i64 = read!();
    let s = format!("{:b}", k);
    for c in s.chars() {
        if c == '1' {
            print!("2");
        } else {
            print!("0");
        }
    }
    println!();
}