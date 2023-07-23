#[macro_use] extern crate text_io;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();
    let c: i32 = read!();

    let ans = 7 - a + 7 - b + 7 - c;
    println!("{}", ans);
}