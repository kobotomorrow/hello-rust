// #[macro_use] extern crate text_io;

fn f(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }

    return f(x - 1) + x;
}

fn main() {
    println!("{}", f(10));
}