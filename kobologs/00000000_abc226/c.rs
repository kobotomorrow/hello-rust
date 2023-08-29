#[macro_use] extern crate text_io;
// use std::cmp::max;
// use std::cmp::min;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let n: usize = read!();
    let mut w: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut t = HashMap::new();
    for i in 0..n {
        let tt: i64 = read!();
        t.insert(i+1, tt);
        
        let mut a = Vec::new();
        let k: usize = read!();
        for _ in 0..k {
            let aa: usize = read!();
            a.push(aa);
        }
        w.insert(i+1, a);
    }
    
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut ans: i64 = t[&n];
    q.push_back(n);
    while !q.is_empty() {
        let x = q.pop_front().unwrap();
        for xx in w.get(&x).unwrap() {
            if !visited.contains(xx) {
                ans += t[xx];
                visited.insert(*xx);
                q.push_back(*xx);
            }
        }
    }
    println!("{}", ans);
}