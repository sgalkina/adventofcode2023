// pub mod day1;
// use crate::day1::*;
// pub mod day2;
// use crate::day2::*;
// pub mod day3;
// use crate::day3::*;
pub mod day4;
use crate::day4::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("./inputs/day4.txt") {
        day4_part2(lines);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}