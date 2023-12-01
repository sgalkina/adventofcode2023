use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn day1(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let digits: Vec<char> = ip.chars().filter(|c| c.is_digit(10)).collect();
            let my_string: String = digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();
            let my_int: i32 = my_string.parse().unwrap();
            result += my_int;
        }
    }
    println!("Result: {}", result);
}


fn find_min_max(string: &String, words: Vec<&str>, min_idx: &mut usize, max_idx: &mut usize, min_val: &mut i32, max_val: &mut i32) {
    for (i, word) in words.iter().enumerate()  {
        let v: Vec<_> = string.match_indices(word).collect();
        for (idx, _occ) in v {
            if idx <= *min_idx {
                *min_idx = idx;
                *min_val = i as i32 +1;
            }
            if idx > *max_idx {
                *max_idx = idx;
                *max_val = i as i32 +1;
            }
        }
    }
}


fn get_first_and_last(string: &String) -> i32 {
    let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut min_idx: usize = string.len();
    let mut max_idx: usize = 0;
    let mut min_val: i32 = 0;
    let mut max_val: i32 = 0;
    find_min_max(string, words, &mut min_idx, &mut max_idx, &mut min_val, &mut max_val);
    find_min_max(string, digits, &mut min_idx, &mut max_idx, &mut min_val, &mut max_val);
    if max_idx <= min_idx {
        max_val = min_val;
    }
    let my_string: String = min_val.to_string() + &max_val.to_string();
    let my_int: i32 = my_string.parse().unwrap();
    my_int
}


fn day1_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            result += get_first_and_last(&ip);
        }
    }
    println!("Result: {}", result);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day1_part2.txt") {
        day1_part2(lines);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}