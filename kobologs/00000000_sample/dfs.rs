// #[macro_use] extern crate text_io;

fn dfs(cur: usize, pattern: &mut [usize]) {
    if cur == pattern.len() {
        println!("{:?}", pattern);
        return;
    }

    pattern[cur] = 0;
    dfs(cur + 1, pattern);

    pattern[cur] = 1;
    dfs(cur + 1, pattern);
}

fn main() {
    let mut pattern = [0; 3];
    dfs(0, &mut pattern);
}