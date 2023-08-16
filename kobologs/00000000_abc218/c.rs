#[macro_use] extern crate text_io;
use std::cmp::{max, min};
//use std::collections::{HashMap, VecDeque, HashSet};

fn snip(s: &Vec<String>) -> Vec<String> {
    let mut xs: usize = 201;
    let mut xe: usize = 0;
    let mut ys: usize = 201;
    let mut ye: usize = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            let c = s[i].chars().nth(j).unwrap();
            if c == '#' {
                xs = min(xs, j);
                xe = max(xe, j);
                ys = min(ys, i);
                ye = max(ye, i);
            }
        }
    }
    return s[ys..ye+1].iter().map(|x| x[xs..xe+1].to_string()).collect();
}

fn rote(s: &Vec<String>) -> Vec<String> {
    let mut t: Vec<String> = vec![];
    for i in 0..s[0].len() {
        let mut ss = String::new();
        for j in (0..s.len()).rev() {
            ss.push(s[j].chars().nth(i).unwrap());
        }
        t.push(ss);
    }
    return t;
}

fn same(s: &Vec<String>, t: &Vec<String>) -> bool {
    if s.len() != t.len() || s[0].len() != t[0].len() {
        return false;
    }
    for i in 0..s.len() {
        if s[i] != t[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let n: usize = read!();
    let s: Vec<String> = (0..n).map(|_| read!()).collect();
    let t: Vec<String> = (0..n).map(|_| read!()).collect();
    let s = snip(&s);
    let t = snip(&t);

    if same(&s, &t) {
        println!("Yes");
        return;
    }
    let s = rote(&s);
    if same(&s, &t) {
        println!("Yes");
        return;
    }
    let s = rote(&s);
    if same(&s, &t) {
        println!("Yes");
        return;
    }
    let s = rote(&s);
    if same(&s, &t) {
        println!("Yes");
        return;
    }
    println!("No");
}
