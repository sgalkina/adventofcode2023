use std::fs::File;
use std::io::{self};

#[derive(Debug)]
struct Tree {
    starts_source: usize,
    ends_source: usize,
    starts_dest: usize,
    left: Option<usize>,
    right: Option<usize>,
}


fn insert_interval(root_idx: Option<usize>, idx: usize, trees: &mut Vec<Tree>) {
    let mut cur_idx: Option<usize> = root_idx;
    while !cur_idx.is_none() {
        // println!("trees: {:#?}", trees);
        let cur = cur_idx.unwrap();
        if trees[idx].ends_source < trees[cur].starts_source {
            cur_idx = match trees[cur].left {
                Some(value) => Some(value),
                None => {
                    trees[cur].left = Some(idx);
                    None
                }
            };
        } else if trees[idx].starts_source > trees[cur].ends_source {
            cur_idx = match trees[cur].right {
                Some(value) => Some(value),
                None => {
                    trees[cur].right = Some(idx);
                    None
                }
            };
        }
    }
}


fn lookup(value: usize, root_idx: Option<usize>, trees: &Vec<Tree>) -> usize {
    match root_idx {
        Some(idx) => {
            if (value >= trees[idx].starts_source) && (value <= trees[idx].ends_source) {
                trees[idx].starts_dest + (value - trees[idx].starts_source)
            } else if value < trees[idx].starts_source {
                lookup(value, trees[idx].left, trees)
            } else {
                lookup(value, trees[idx].right, trees)
            }
        },
        None => value
    }
}


fn parse_map(lines: &Vec<String>) -> Vec<Tree> {
    let mut trees: Vec<Tree> = vec![];
    let vals: Vec<Vec<usize>> = lines.iter().map(|s| 
        s.trim().split_whitespace()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect())
    .collect();
    for (i, val) in vals.iter().enumerate() {
        trees.push(Tree {
            starts_source: val[1],
            ends_source: val[1] + val[2] - 1,
            starts_dest: val[0],
            left: None,
            right: None,
        });
        if i > 0 {
            insert_interval(Some(0), i, &mut trees);
        }
    }
    trees
}


pub fn day5(lines: std::io::Lines<io::BufReader<File>>) {
    let mut seeds: Vec<usize> = vec![];
    let mut current_lines: Vec<String> = vec![];
    let mut all_trees: Vec<Vec<Tree>> = vec![];
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            if x == 0 {
                seeds = ip.split(":")
                .last().unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect();
                continue;
            }
            let to_collect: bool = match ip.trim().chars().last() {
                Some(':') => false,
                Some(_) => true,
                None => false
            };
            if to_collect {
                current_lines.push(ip);
            } else {
                if !current_lines.is_empty() {
                    all_trees.push(parse_map(&current_lines));
                    current_lines = vec![];
                }
            }
        }
    }
    all_trees.push(parse_map(&current_lines));
    // Part 1
    // let mut locs = vec![];
    // for seed in seeds {
    //     let mut lookup_index = seed;
    //     for trees in &all_trees {
    //         lookup_index = lookup(lookup_index, Some(0), trees);
    //     }
    //     locs.push(lookup_index);
    // }
    // Part 2
    let mut locs = vec![];
    for x in (0..seeds.len()).step_by(2) {
        for seed in seeds[x]..seeds[x]+seeds[x+1] {
            let mut lookup_index = seed;
            for trees in &all_trees {
                lookup_index = lookup(lookup_index, Some(0), trees);
            }
            locs.push(lookup_index);
        }
    }
    println!("Result: {:?}", locs.iter().min().unwrap());
}
