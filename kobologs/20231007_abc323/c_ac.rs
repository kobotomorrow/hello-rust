#[macro_use] extern crate text_io;
use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::collections::HashSet;
// use num_bigint::BigInt;

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let a = (0..m).map(|_| read!()).collect::<Vec<usize>>();
    let s = (0..n).map(|_| read!()).collect::<Vec<String>>();

    let mut cur_scores = Vec::new();
    let mut unsoluved = Vec::new();
    let mut max_score = 0;


    for (i, ss) in s.iter().enumerate() {
        let mut cur_score = 0;
        let mut uns = Vec::new();
        for (j, c) in ss.chars().enumerate() {
            if c == 'o' {
                cur_score += a[j];
            } else {
                uns.push(a[j]);
            }
        }
        max_score = max(max_score, cur_score+i+1);
        cur_scores.push(cur_score+i+1);
        uns.sort_by(|a, b| b.cmp(a));
        unsoluved.push(uns);
    }

    let mut ans = HashMap::new();
    for i in 0..n {
        if cur_scores[i] == max_score {
            ans.insert(i, 0);
            continue;
        }
        let mut cur_score = cur_scores[i];
        for (j, x) in unsoluved[i].iter().enumerate() {
            cur_score += x;
            if cur_score > max_score {
                ans.insert(i, j+1);
                break;
            }
        }
    }

    for i in 0..n {
        println!("{}", ans.get(&i).unwrap());
    }
}