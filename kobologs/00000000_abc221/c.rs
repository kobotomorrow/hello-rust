#[macro_use] extern crate text_io;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: String = read!();
    let n: Vec<char> = n.chars().collect::<Vec<char>>();
    let size = n.len();
    let mut result = HashSet::new();
    permute(n, 0, size - 1, &mut result);
    let mut ans = 0;
    for s in result {
        ans = max(ans, calc(&s));
    }
    println!("{}", ans);
}

fn calc(s: &str) -> i64 {
    let mut res: i64 = 0;
    for i in 1..s.len() {
        let s1 = &s[0..i];
        let s2 = &s[i..s.len()];
        let s1 = s1.parse::<i64>().unwrap();
        let s2 = s2.parse::<i64>().unwrap();
        res = max(res, s1 * s2);
    }

    return res;
}

fn permute(s: Vec<char>, l: usize, r: usize, result: &mut HashSet<String>) {
    if l == r {
        result.insert(s.iter().collect());
    } else {
        for i in l..=r {
            let mut s = s.clone();
            s.swap(l, i);
            permute(s, l + 1, r, result);
        }
    }
}
