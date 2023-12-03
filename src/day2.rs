use std::fs::File;
use std::io::{self};
extern crate regex;
use regex::Regex;


fn match_color(string: &str, color: &str) -> Vec<i32> {
    let mut digits = vec![];
    let re = Regex::new(format!(r#"(\d+) {}"#, color).as_str()).unwrap();
    for (_, [digit]) in re.captures_iter(string).map(|caps| caps.extract()) {
        digits.push(digit.parse::<i32>().unwrap());
    }
    digits
}


pub fn day2(lines: std::io::Lines<io::BufReader<File>>) {
    let colors = vec!["red", "green", "blue"];
    let total = vec![12, 13, 14];
    let mut result: i32 = 0;
    let mut to_add: bool = true;
    for (idx, line) in lines.enumerate() {
        if let Ok(ip) = line {
            for (i, color) in colors.iter().enumerate() {
                let digits: Vec<i32> = match_color(&ip, color);
                for digit in digits {
                    if total[i] < digit {
                        to_add = false;
                    }
                }
            }
            if to_add {
                result += idx as i32 + 1;
            }
            to_add = true;
        }
    }
    println!("Result: {}", result);
}


pub fn day2_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let colors = vec!["red", "green", "blue"];
    let mut result: i32 = 0;
    for (idx, line) in lines.enumerate() {
        if let Ok(ip) = line {
            let mut maxes: Vec<i32> = vec![0, 0, 0];
            for (i, color) in colors.iter().enumerate() {
                let digits: Vec<i32> = match_color(&ip, color);
                for digit in digits {
                    if maxes[i] <= digit {
                        maxes[i] = digit;
                    }
                }
            }
            result += maxes[0]*maxes[1]*maxes[2];
        }
    }
    println!("Result: {}", result);
}
