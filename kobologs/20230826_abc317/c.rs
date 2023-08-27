#[macro_use] extern crate text_io;
use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
// use std::collections::VecDeque;
use std::collections::HashSet;

fn dfs(cur: usize, r: &HashMap<usize, Vec<(usize, i64)>>, visited: &mut HashSet<usize>, dist: i64, d_max: &mut i64) {
    visited.insert(cur);
    if !r.contains_key(&cur) {
        return;
    }
    let e = r.get(&cur).unwrap();
    for (next, d) in e {
        if visited.contains(next) {
            *d_max = max(*d_max, dist);
        } else {
            dfs(*next, r, visited, dist + d, d_max);
            visited.remove(next);
        }
    }
}

fn main() {
    let n: usize = read!();
    let m: usize = read!();

    let mut r: HashMap<usize, Vec<(usize, i64)>> = HashMap::new();
    for _ in 0..m {
        let a: usize = read!();
        let b: usize = read!();
        let c: i64 = read!();

        if !r.contains_key(&a) {
            let mut v: Vec<(usize, i64)> = Vec::new();
            v.push((b, c));
            r.insert(a, v);
        } else {
            let e = r.get_mut(&a).unwrap();
            e.push((b, c));
        }
        if !r.contains_key(&b) {
            let mut v: Vec<(usize, i64)> = Vec::new();
            v.push((a, c));
            r.insert(b, v);
        } else {
            let e = r.get_mut(&b).unwrap();
            e.push((a, c));
        }
    }

    let mut d_max: i64 = 0;
    let d: i64 = 0;
    for i in 1..n {
        let mut visited: HashSet<usize> = HashSet::new();
        dfs(i, &r, &mut visited, d, &mut d_max);
    }
    println!("{}", d_max);
}