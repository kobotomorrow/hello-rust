// ABC168 D
#[macro_use] extern crate text_io;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let n: i32 = read!();
    let m: i32 = read!();

    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for _ in 0..m {
        let a: i32 = read!();
        let b: i32 = read!();

        g.entry(a).or_insert(Vec::new()).push(b);
        g.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut ans = vec![0; n as usize - 1];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(1);
    queue.push_back(1);

    while !queue.is_empty() {
        let now = queue.pop_front().unwrap();
        for v in g.get(&now).unwrap() {
            if visited.contains(v) {
                continue;
            }
            let a = v - 2;
            ans[a as usize] = now;
            visited.insert(*v);
            queue.push_back(*v);
        }
    }

    for &a in &ans {
        if a == 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    for a in ans {
        println!("{}", a);
    }
}