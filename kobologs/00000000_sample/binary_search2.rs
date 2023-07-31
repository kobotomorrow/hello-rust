// Tessoku: A13
#[macro_use] extern crate text_io;

fn main() {
    let n: i32 = read!();
    let k: i32 = read!();
    let a: Vec<i32> = (0..n).map(|_| read!()).collect();

    let mut ans: i64 = 0;

    for i in 0..n-1 {
        let mut l = i + 1;
        let mut r = n;

        while l + 1 < r {
            let m = (l + r) / 2;
            if a[m as usize] - a[i as usize] <= k {
                l = m;
            } else {
                r = m;
            }
        }

        if a[l as usize] - a[i as usize] <= k {
            ans += l as i64 - i as i64;
        }
    }

    println!("{}", ans);
}