#[macro_use] extern crate text_io;

fn is_match(s: String, mut x: i32) -> bool {
    let mut input = vec![false; 10];
    for _i in 0..4 {
        let a = x % 10;
        input[a as usize] = true;
        x /= 10;
    }

    for i in 0..10 {
        let c = s.chars().nth(i);
        if c == Some('o') && !input[i] {
            return false;
        }
        if c == Some('x') && input[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let s: String = read!();
    let mut ans = 0;
    for i in 0..10000 {
        if is_match(s.clone(), i) {
            ans += 1;
        }
    }
    println!("{}", ans);
}