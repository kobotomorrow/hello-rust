#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let q: usize = read!();
    let mut a: Vec<i32> = (0..n).map(|_| read!()).collect();
    a.sort();
    let x: Vec<i32> = (0..q).map(|_| read!()).collect();

    for xx in x.iter() {
        let tg = match a.binary_search(xx) {
            Ok(i) => i,
            Err(i) => i,
        };
        println!("{}", n-tg);
    }
}