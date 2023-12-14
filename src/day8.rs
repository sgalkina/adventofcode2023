use std::fs::File;
use std::io::{self};


#[derive(Debug)]
struct Tree {
    content: String,
    left: Option<usize>,
    right: Option<usize>,
}


fn push_tree(node: String, trees: &mut Vec<Tree>) -> usize {
    let mut idx = trees.len();
    match trees.iter().position(|e| *e.content == node) {
        Some(v) => idx = v,
        None => trees.push(Tree {content: node, left: None, right: None})
    }
    idx
}


fn insert(node: String, left: String, right: String, trees: &mut Vec<Tree>) {
    let left_idx = push_tree(left, trees);
    let right_idx = push_tree(right, trees);
    let idx = push_tree(node, trees);
    trees[idx].left = Some(left_idx);
    trees[idx].right = Some(right_idx);
}


pub fn day8(lines: std::io::Lines<io::BufReader<File>>) {
    let mut instr: String = "".to_string();
    let mut trees: Vec<Tree> = vec![];
    for (x, line) in lines.enumerate() {
        if let Ok(ip) = line {
            if x == 0 {
                instr = ip.trim().to_string();
                continue;
            }
            if x == 1 {continue;}
            let vecs: Vec<String> = ip.split('=')
                .map(|s| s.trim().to_string())
                .collect();
            let branches: Vec<String> = vecs[1].replace(['(', ')'], "").split(", ").map(|x| x.to_string()).collect();
            insert(vecs[0].clone(), branches[0].clone(), branches[1].clone(), &mut trees);
        }
    }
    let mut count: usize = 0;
    let mut opt: Option<usize> = None;
    for (i, tree) in trees.iter().enumerate() {
        if tree.content == "AAA" {
            opt = Some(i);
        }
    }
    let mut root_id = match opt {
        Some(a) => a,
        None => panic!("AAA not found")
    };
    // println!("Result: {:?}", trees);
    for c in instr.chars().cycle() {
        // println!("node: {:?}", trees[root_id].content);
        if trees[root_id].content == "ZZZ" {
            break;
        }
        let idx = match c {
            'L' => trees[root_id].left,
            'R' => trees[root_id].right,
            _ => panic!("Incorrect intructions")
        };
        root_id = idx.unwrap();
        count += 1;
    }
    println!("Result: {:?}", count);
}
    
