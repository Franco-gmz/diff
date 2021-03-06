//! This module contains some public functions to calculate and print diff, taking string vectors.
use crate::args::*;
use crate::errors::*;
use crate::file::*;

///Longest common subsequence. It receives two references to Vec<String>
/// and returns a grid with the results of the algorithm.
fn lcs(a: &[String], b: &[String]) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; b.len() + 1]; a.len() + 1];
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

///It prints the diff results on stdout taking the result of lcs like input.
fn print_diff(grid: &[Vec<i32>], a: &[String], b: &[String], i: usize, j: usize) {
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

///Execute file diff by lines, taking the file paths from console. Then it prints the results on stdout.
pub fn run() -> std::result::Result<(), Errors> {
    /* arguments */
    let args: Arguments = Arguments::new()?;
    let filename1: &String = &args.arg1;
    let filename2: &String = &args.arg2;
    /* read files */
    let content1: Vec<String> = read_file_lines(filename1)?;
    let content2: Vec<String> = read_file_lines(filename2)?;
    /* diff */
    let grid: Vec<Vec<i32>> = lcs(&content1, &content2);
    let length1 = content1.len();
    let length2 = content2.len();
    print_diff(&grid, &content1, &content2, length1, length2);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::fdiff::lcs;

    #[test]
    fn lcs_test() {
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
