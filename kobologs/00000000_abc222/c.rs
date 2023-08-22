#[macro_use] extern crate text_io;
// use std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let t: Vec<String> = (0..n*2).map(|_| read!()).collect();
    let mut rt: Vec<(usize, i32)> = (0..n*2).map(|i| (i, 0)).collect();
    
    for i in 0..m {
        let mut o = rt.clone();
        o.sort_by_key(|a| -a.1);
        for vs in o.chunks(2) {
            let l = vs[0];
            let r = vs[1];
            let result = gcp(t[l.0].chars().nth(i).unwrap(), t[r.0].chars().nth(i).unwrap());
            if result == 'l' {
                rt[l.0].1 += 1;
            } else if result == 'r' {
                rt[r.0].1 += 1;
            }
        }
    }

    rt.sort_by_key(|a| -a.1);
    for i in 0..n*2 {
        println!("{}", rt[i].0 + 1);
    }
}

fn gcp(l: char, r: char) -> char {
    if l == 'G' && r == 'C' {
        'l'
    } else if l == 'C' && r == 'P' {
        'l'
    } else if l == 'P' && r == 'G' {
        'l'
    } else if l == 'G' && r == 'P' {
        'r'
    } else if l == 'C' && r == 'G' {
        'r'
    } else if l == 'P' && r == 'C' {
        'r'
    } else {
        'd'
    }
}