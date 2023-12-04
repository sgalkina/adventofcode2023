use std::fs::File;
use std::io::{self};
use std::collections::HashSet;


pub fn day4(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            let mut result_card: i32 = 0;
            let numbers_sets: Vec<Vec<i32>> = ip.split(":")
                .last().unwrap()
                .trim().split("|")
                .map(|s| s.trim()
                    .split_whitespace()
                    .map(|x| x.to_string().parse::<i32>().unwrap())
                    .collect()
                )
                .collect();
            let winning: HashSet<i32> = HashSet::from_iter(numbers_sets[0].clone());
            numbers_sets[1].iter()
                .filter(|s| winning.contains(&s))
                .inspect(|_| result_card = if result_card == 0 { 1 } else { result_card*2 })
                .collect::<Vec<&i32>>();
            result += result_card;
        }
    }
    println!("Result: {}", result);
}
