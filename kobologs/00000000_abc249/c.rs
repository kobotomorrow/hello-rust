#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::VecDeque;
use std::collections::HashSet;
// use num_bigint::BigInt;

fn dfs(cur: usize, pattern: &mut [usize], s: &Vec<HashSet<char>>, targets: &mut Vec<Vec<HashSet<char>>>) {
    if cur == pattern.len() {
        let mut tmp = Vec::new();
        for i in 0..pattern.len() {
            if pattern[i] == 1 {
                tmp.push(s[i].clone());
            }
        }
        if tmp.len() == 0 {
            return;
        }
        targets.push(tmp);
        return;
    }

    pattern[cur] = 0;
    dfs(cur + 1, pattern, s, targets);

    pattern[cur] = 1;
    dfs(cur + 1, pattern, s, targets);
}

fn main() {
    let n: usize = read!();
    let k: i32 = read!();
    let mut s = Vec::new();
    for _ in 0..n {
        let ss: String = read!();
        s.push(ss.chars().collect::<HashSet<char>>());
    }

    let mut pattern = vec![0; n];
    let mut targets = Vec::new();
    dfs(0, &mut pattern, &s, &mut targets);

    let mut ans = 0;
    for target in targets {
        let mut count = HashMap::new();
        for t in target {
            for c in 'a'..='z' {
                if t.contains(&c) {
                    *count.entry(c).or_insert(0) += 1;
                }
            }
        }
        
        let mut tmp = 0;
        for cnt in count.values() {
            if *cnt == k {
                tmp += 1;
            }
        }
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
