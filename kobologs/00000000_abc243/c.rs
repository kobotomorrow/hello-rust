use std::hash::Hash;

#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn check(xs: &mut Vec<(i32, usize)>, s: &Vec<char>) -> bool {
    xs.sort();
    let mut find_c = 'R';
    for i in 0..xs.len() {
        if find_c == 'R' && s[xs[i].1] == 'R' {
            find_c = 'L';
        }
        if find_c == 'L' && s[xs[i].1] == 'L' {
            return true
        }
    }
    return false
}

fn main() {
    let n: usize = read!();
    let mut z = HashMap::new();
    for i in 0..n {
        let x :i32 = read!();
        let y :i32 = read!();

        if z.contains_key(&y) {
            let xs: &mut Vec<(i32, usize)> = z.get_mut(&y).unwrap();
            xs.push((x, i));
        } else {
            z.insert(y, vec![(x, i)]);
        }
    }

    let s: String = read!();
    let s: Vec<char> = s.chars().collect();

    for (_, mut xs) in z {
        if xs.len() < 2 {
            continue
        }
        if check(&mut xs, &s) {
            println!("Yes");
            return
        }
    }
    println!("No");
}
