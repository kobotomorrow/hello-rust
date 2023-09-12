#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let s: Vec<String> = (0..n).map(|_| read!()).collect();
    let mut t = HashSet::new();
    for _ in 0..m {
        let tt: String = read!();
        t.insert(tt);
    }
    for ss in s.iter() {
        if t.contains(ss) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}