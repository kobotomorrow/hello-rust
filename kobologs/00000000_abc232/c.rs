#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
use std::collections::HashSet;

fn permute(s: Vec<usize>, l: usize, r: usize, result: &mut HashSet<Vec<usize>>) {
    if l == r {
        result.insert(s);
    } else {
        for i in l..=r {
            let mut s = s.clone();
            s.swap(l, i);
            permute(s, l + 1, r, result);
        }
    }
}

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let mut ga = vec![vec![false; n]; n];
    let mut gb = vec![vec![false; n]; n];
    for _ in 0..m {
        let a: usize = read!();
        let b: usize = read!();
        ga[a - 1][b - 1] = true;
        ga[b - 1][a - 1] = true;
    }
    for _ in 0..m {
        let c: usize = read!();
        let d: usize = read!();
        gb[c - 1][d - 1] = true;
        gb[d - 1][c - 1] = true;
    }
    if ga == gb {
        println!("Yes");
        return;
    }

    let v: Vec<usize> = (0..n).map(|x| x).collect();
    let l = v.len();
    let mut ps = HashSet::new();
    permute(v, 0, l - 1, &mut ps);

    for p in ps.iter() {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if ga[i][j] != gb[p[i]][p[j]] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}