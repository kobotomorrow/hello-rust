#[macro_use] extern crate text_io;
use std::cmp::max;
use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;

fn main() {
    let _: i64 = read!();
    let a: i64 = read!();
    let b: i64 = read!();
    let p: i64 = read!();
    let q: i64 = read!();
    let r: i64 = read!();
    let s: i64 = read!();

    let mut ans = vec![vec!['.'; (s-r+1)as usize]; (q-p+1)as usize];

    let st = max(p-a, r-b);
    let en = min(q-a, s-b);

    for i in st..=en {
        ans[(i+a-p)as usize][(i+b-r)as usize] = '#';
    }

    let st = max(p-a, b-s);
    let en = min(q-a, b-r);

    for i in st..=en {
        ans[(i+a-p)as usize][(b-i-r)as usize] = '#';
    }

    for aa in ans {
        println!("{}", aa.iter().collect::<String>());
    }
}