#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let squire: Vec<String> = (0..n).map(|_| read!()).collect();
    let squire: Vec<Vec<char>> = squire.iter().map(|x| x.chars().collect()).collect();

    for i in 0..n {
        for j in 0..n-5 {
            let mut cnt = 0;
            for k in 0..6 {
                if squire[i][j+k] == '#' {
                    cnt += 1;
                }
            }
            if cnt >= 4 {
                println!("Yes");
                return;
            }

            let mut cnt = 0;
            for k in 0..6 {
                if squire[j+k][i] == '#' {
                    cnt += 1;
                }
            }
            if cnt >= 4 {
                println!("Yes");
                return;
            }

            if i < n-5 {
                let mut cnt1 = 0;
                let mut cnt2 = 0;
                for k in 0..6 {
                    if squire[j+k][i+k] == '#' {
                        cnt1 += 1;
                    }
                    if squire[j+k][i+5-k] == '#' {
                        cnt2 += 1;
                    }
                }
                if cnt1 >= 4 || cnt2 >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}