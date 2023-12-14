use std::fs::File;
use std::io::{self};

fn process(list: Vec<i32>) -> i32 {
    if list.iter().sum::<i32>() == 0 {
        return 0;
    }
    let intervals: Vec<i32> = (1..list.len()).map(|i| list[i] - list[i - 1]).collect();
    return *list.last().unwrap() + process(intervals);
}

fn process_backwards(list: Vec<i32>) -> i32 {
    if list.iter().sum::<i32>() == 0 {
        return 0;
    }
    let intervals: Vec<i32> = (1..list.len()).map(|i| list[i] - list[i - 1]).collect();
    return *list.first().unwrap() - process_backwards(intervals);
}

pub fn day9(lines: std::io::Lines<io::BufReader<File>>) {
    let mut res: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let numbers: Vec<i32> = ip
                .split_whitespace()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect();
            res += process_backwards(numbers);
        }
    }
    println!("Result: {:?}", res);
}
