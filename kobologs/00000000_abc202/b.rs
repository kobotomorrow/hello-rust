#[macro_use] extern crate text_io;

fn main() {
    let s: String = read!();

    for c in s.chars().rev() {
        let mut cc = c;
        if c == '6' {
            cc = '9';
        } else if c == '9' {
            cc = '6';
        }
        print!("{}", cc);
    }
    println!();
}