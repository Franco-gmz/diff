use diff::fdiff;
use diff::errors::Errors;

///It receives two file paths by console and calculate the differences between their content.
fn main() -> std::result::Result<(), Errors>{
    fdiff::run()
}
