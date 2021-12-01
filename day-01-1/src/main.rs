use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count = 0;
    let mut previous: Option<i32> = None;
    let lines = read_lines("./input.txt").expect("Could not open file");
    for line in lines {
        let current = line.expect("Invalid line").parse::<i32>().expect("Invalid number");
        if previous.is_some() {
            if current > previous.unwrap() {
                count += 1;
            }
        }
        previous = Some(current);
    }
    println!("{}", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
