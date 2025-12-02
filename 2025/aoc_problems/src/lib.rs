use std::fs::File;
use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html?highlight=read%20lines#a-naive-approach
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    return read_to_string(filename);
}
