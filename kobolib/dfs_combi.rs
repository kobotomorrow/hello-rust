fn dfs_combi(depth: usize, size: usize, min: usize, max: usize, combination: &mut [usize]) {
    if depth == size {
        println!("{:?}", combination);
    } else {
        for i in min..=max {
            combination[depth] = i;
            dfs_combi(depth + 1, size, i, max, combination);
        }
    }
}

fn main() {
    let size = 3;
    let min = 1;
    let max = 3;
    let mut combination = [0; 3];
    dfs_combi(0, size, min, max, &mut combination);
}