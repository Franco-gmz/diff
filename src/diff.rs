///Longest common subsequence. It receives two references to Vec<String>
/// and returns a grid with the results of the algorithm.
pub fn lcs(a: &[String], b: &[String]) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; a.len() + 1]; b.len() + 1];
    for i in 0..a.len() {
        //for j in 0..b.len() {
        for (j, _item) in b.iter().enumerate() {
            if a[i] == b[j] {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else if grid[i + 1][j] > grid[i][j + 1] {
                grid[i + 1][j + 1] = grid[i + 1][j];
            } else {
                grid[i + 1][j + 1] = grid[i][j + 1];
            }
        }
    }
    grid
}

///It prints the diff results on stdout.
pub fn print_diff(grid: &[Vec<i32>], a: &[String], b: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        print_diff(grid, a, b, i - 1, j - 1);
        println!("  { }", a[i - 1]);
    } else if j > 0 && (i == 0 || grid[i][j - 1] >= grid[i - 1][j]) {
        print_diff(grid, a, b, i, j - 1);
        println!("> { }", b[j - 1]);
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        print_diff(grid, a, b, i - 1, j);
        println!("< { }", a[i - 1]);
    } else {
        println!();
    }
}
