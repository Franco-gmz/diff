///Longest common subsequence. It receives two references to Vec<String>
/// and returns a grid with the results of the algorithm.
pub fn lcs(a: &[String], b: &[String]) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; a.len() + 1]; b.len() + 1];
    for i in 0..a.len() {
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

#[cfg(test)]
mod test{
    use crate::diff::lcs;

    #[test]
    fn lcs_test(){
        //lines1
        let mut lines1: Vec<String> = Vec::new();
        lines1.push("abcd".to_string());
        lines1.push("aaa".to_string());
        lines1.push("fff".to_string());
        //lines2
        let mut lines2: Vec<String> = Vec::new();
        lines2.push("acde".to_string());
        lines2.push("bbb".to_string());
        lines2.push("fff".to_string());
        //expected grid
        let mut expected_grid = vec![vec![0; 4]; 4];
        expected_grid[3][3] = 1;
        //lcs
        let grid = lcs(&lines1, &lines2);
        assert_eq!(grid, expected_grid);
    }
}
