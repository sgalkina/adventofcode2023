// pub mod day1;
// use crate::day1::*;
// pub mod day2;
// use crate::day2::*;
// pub mod day3;
// use crate::day3::*;
// pub mod day4;
// use crate::day4::*;
// pub mod day5;
// use crate::day5::*;
// pub mod day6;
// use crate::day6::*;
// pub mod day7;
// use crate::day7::*;
// pub mod day8;
// use crate::day8::*;
pub mod day9;
use crate::day9::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./inputs/day9.txt") {
        day9(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
