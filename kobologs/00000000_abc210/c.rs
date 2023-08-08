#[macro_use] extern crate text_io;
use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::{collections::HashSet, hash::Hash};

fn plus(cnt: &mut HashMap<i32, i32>, key: i32) {
    cnt.entry(key).or_insert(0);
    cnt.insert(key, cnt.get(&key).unwrap() + 1);
}

fn minus(cnt: &mut HashMap<i32, i32>, key: i32) {
    match cnt.get(&key) {
        Some(x) => {
            if *x == 1 {
                cnt.remove(&key);
            } else {
                cnt.insert(key, *x - 1);
            }
        },
        None => {}
    }
}

fn main() {
    let n: i32 = read!();
    let k: i32 = read!();
    let c: Vec<i32> = (0..n).map(|_| read!()).collect();

    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    for i in 0..n {
        if i < k {
            plus(&mut cnt, c[i as usize]);
            ans = cnt.len() as i32;
            continue;
        }
        plus(&mut cnt, c[i as usize]);
        minus(&mut cnt, c[(i - k) as usize]);
        ans = max(ans, cnt.len() as i32);
    }
    println!("{}", ans);
}