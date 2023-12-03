use std::fs::File;
use std::io::{self};
extern crate regex;
use regex::Regex;


#[derive(Debug)]
struct Box {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    content: String,
}


fn intersect(box1: &Box, box2: &Box) -> bool {
    let x_overlaps = box1.max_x >= box2.min_x && box2.max_x >= box1.min_x;
    let y_overlaps = box1.max_y >= box2.min_y && box2.max_y >= box1.min_y;
    x_overlaps && y_overlaps
}


fn build_box(min_x: usize, max_x: usize, min_y: usize, max_y: usize, content: String) -> Box {
    Box {min_x, max_x, min_y, max_y, content}
}


fn number_boxes(line: &String, x: usize) -> Vec<Box> {
    let re = Regex::new(r#"(\d+)"#).unwrap();
    let boxes: Vec<Box> = re.find_iter(line).map(|m| build_box(x, x, m.start(), m.end()-1, (&m.as_str()).to_string())).collect();
    boxes
}


fn symbol_boxes(line: &String, x: usize) -> Vec<Box> {
    let re = Regex::new(r#"[^.0-9]"#).unwrap();
    let boxes: Vec<Box> = re.find_iter(line).map(|m| build_box(x-1, x+1, m.start()-1, m.end(), (&m.as_str()).to_string())).collect();
    boxes
}


pub fn day3(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    let mut numbers: Vec<Box> = vec![];
    let mut symbols: Vec<Box> = vec![];
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            numbers.extend(number_boxes(&ip, x));
            symbols.extend(symbol_boxes(&ip, x));
        }
    }
    for number in &numbers {
        for symbol in &symbols {
            if intersect(number, symbol) {
                result += number.content.parse::<i32>().unwrap();
                break;
            }
        }
    }
    println!("Result: {}", result);
}


pub fn day3_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    let mut numbers: Vec<Box> = vec![];
    let mut symbols: Vec<Box> = vec![];
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            numbers.extend(number_boxes(&ip, x));
            symbols.extend(symbol_boxes(&ip, x));
        }
    }
    for symbol in &symbols {
        if symbol.content == "*" {
            let mut adj_numbers: Vec<&Box> = vec![];
            for number in &numbers {
                if intersect(number, symbol) {
                    adj_numbers.push(&number);
                }
            }
            if adj_numbers.len() == 2 {
                result += adj_numbers[0].content.parse::<i32>().unwrap()*adj_numbers[1].content.parse::<i32>().unwrap();
            }
        }
    }
    println!("Result: {}", result);
}