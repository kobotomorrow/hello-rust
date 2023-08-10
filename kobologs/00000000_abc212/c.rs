#[macro_use] extern crate text_io;
use std::cmp::{max, min};
// use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let n: i32 = read!();
    let m: i32 = read!();
    let mut a = (0..n).map(|_| read!()).collect::<Vec<i32>>();
    a.sort();
    let mut b = (0..m).map(|_| read!()).collect::<Vec<i32>>();
    b.sort();

    let mut mn = 1_000_000_000;
    for aa in a {
        let x = b.binary_search(&aa);
        match x {
            Ok(_) => {
                println!("{}", 0);
                return;
            },
            Err(i) => {
                if i == 0 {
                    mn = min(mn, (aa - b[i]).abs());
                    continue;
                }
                if i == b.len() {
                    mn = min(mn, (aa - b[i-1]).abs());
                    continue;
                }
                let z = min((aa - b[i]).abs(), (aa - b[i-1]).abs());
                mn = min(mn, z);
            }
        }
    }
    println!("{}", mn);
}