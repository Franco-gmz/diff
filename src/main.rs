mod args;
mod diff;
mod file;

///It receives two file paths by console and calculate the differences between their content.
fn main() -> Result<(), String> {
    /* arguments */
    let args: args::Arguments = args::Arguments::new()?;
    let filename1: &String = &args.arg1;
    let filename2: &String = &args.arg2;
    /* read files */
    let content1: Vec<String> = file::read_file_lines(filename1)?;
    let content2: Vec<String> = file::read_file_lines(filename2)?;
    /* diff */
    let grid: Vec<Vec<i32>> = diff::lcs(&content1, &content2);
    let length1 = content1.len();
    let length2 = content2.len();
    diff::print_diff(&grid, &content1, &content2, length1, length2);
    Ok(())
}
