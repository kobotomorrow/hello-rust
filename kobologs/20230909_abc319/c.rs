#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::{collections::HashMap};
// use std::collections::VecDeque;
use std::collections::HashSet;

fn check(p: Vec<(i32, usize, usize)>) -> bool {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    let mut col3 = Vec::new();
    let mut row1 = Vec::new();
    let mut row2 = Vec::new();
    let mut row3 = Vec::new();
    let mut cross1 = Vec::new();
    let mut cross2 = Vec::new();

    for pp in p {
        let (n, y, x) = pp;
        if y == 0 {
            row1.push(n);
        } else if y == 1 {
            row2.push(n);
        } else {
            row3.push(n);
        }
        if x == 0 {
            col1.push(n);
        } else if x == 1 {
            col2.push(n);
        } else {
            col3.push(n);
        }
        if x == 0 && y == 0 || x == 1 && y == 1 || x == 2 && y == 2 {
            cross1.push(n);
        }
        if x == 0 && y == 2 || x == 1 && y == 1 || x == 2 && y == 0 {
            cross2.push(n);
        }
    }
    if col1.len() == 3 {
        if col1[0] == col1[1] {
            return false
        }
    }
    if col2.len() == 3 {
        if col2[0] == col2[1] {
            return false
        }
    }
    if col3.len() == 3 {
        if col3[0] == col3[1] {
            return false
        }
    }
    if row1.len() == 3 {
        if row1[0] == row1[1] {
            return false
        }
    }
    if row2.len() == 3 {
        if row2[0] == row2[1] {
            return false
        }
    }
    if row3.len() == 3 {
        if row3[0] == row3[1] {
            return false
        }
    }
    if cross1.len() == 3 {
        if cross1[0] == cross1[1] {
            return false
        }
    }
    if cross2.len() == 3 {
        if cross2[0] == cross2[1] {
            return false
        }
    }
    return true
}

fn permute(s: Vec<(i32, usize, usize)>, l: usize, r: usize, result: &mut HashSet<Vec<(i32, usize, usize)>>) {
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
    let mut v = vec![vec![0;3];3];
    for i in 0..3 {
        for j in 0..3 {
            let n: i32 = read!();
            v[i][j] = n;
        }
    }
    let mut target = HashSet::new();
    for i in 0..3 {
        if v[i][0] == v[i][1] || v[i][1] == v[i][2] || v[i][0] == v[i][2] {
            target.insert((v[i][0], i, 0));
            target.insert((v[i][1], i, 1));
            target.insert((v[i][2], i, 2));
        }
        if v[0][i] == v[1][i] || v[1][i] == v[2][i] || v[0][i] == v[2][i] {
            target.insert((v[0][i], 0, i));
            target.insert((v[1][i], 1, i));
            target.insert((v[2][i], 2, i));
        }
    }
    if v[0][0] == v[1][1] || v[1][1] == v[2][2] || v[0][0] == v[2][2] {
        target.insert((v[0][0], 0, 0));
        target.insert((v[1][1], 1, 1));
        target.insert((v[2][2], 2, 2));
    }
    if v[0][2] == v[1][1] || v[1][1] == v[2][0] || v[0][2] == v[2][0] {
        target.insert((v[0][2], 0, 2));
        target.insert((v[1][1], 1, 1));
        target.insert((v[2][0], 2, 0));
    }
    
    let mut result = HashSet::new();
    if target.len() == 0 {
        println!("1");
        return
    }
    permute(target.iter().cloned().collect(), 0, target.len()-1, &mut result);
    
    let mut count: i64 = 0;
    for r in result {
        if check(r) {
            count += 1;
        }
    }

    let mut bb: i64 = 1;
    for i in 1..target.len()+1 {
        bb *= i as i64;
    }

    let ans = count as f64 / bb as f64;
    println!("{}", ans);
}