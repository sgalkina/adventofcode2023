use std::fs::File;
use std::io::{self};


pub fn day6(lines: std::io::Lines<io::BufReader<File>>) {
    let mut times: Vec<usize> = vec![];
    let mut records: Vec<usize> = vec![];
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            let numbers: Vec<usize> = ip.split(":")
            .last().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect();
            match x {
                0 => times = numbers.clone(),
                1 => records = numbers.clone(),
                _ => panic!("More than two lines in the file")
            }
        }
    }
    let result: Vec<usize> = times.into_iter().enumerate()
        .map(|(i, x)| (1..=x)
                                        .map(|a| a*(x-a))
                                        .filter(|s| *s > records[i])
                                        .count()
            ).collect();
    let res: usize = result.into_iter().reduce(|a, b| a*b).unwrap();
    println!("Result: {:?}", res);
}


pub fn day6_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut time: usize = 0;
    let mut record: usize = 0;
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            let number: usize = ip.split(":")
            .last().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.to_string()).collect::<Vec<_>>().join("").parse::<usize>().unwrap();
            match x {
                0 => time = number,
                1 => record = number,
                _ => panic!("More than two lines in the file")
            }
        }
    }
    let result: usize = (1..=time)
        .map(|a| a*(time-a))
        .filter(|s| *s > record)
        .count();
    println!("Result: {:?}", result);
}