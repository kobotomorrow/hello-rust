#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
use std::collections::BTreeMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let mut sl = BTreeMap::new();
    for _ in 0..n {
        let s: u64 = read!();
        let c: u64 = read!();
        sl.insert(s, c);
    }
    let mut cnt = 0;
    while let Some((s, c)) = sl.pop_first() {
        if c % 2 == 1 {
            cnt += 1;
        }
        if c >= 2 {
            *sl.entry(s * 2).or_default() += c / 2;
        }
    }
    println!("{}", cnt);
}